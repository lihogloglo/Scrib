# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1412 frames over 15.0 s · **median 10.54 ms (95 fps)** · mean 10.63 · p95 11.80 · p99 12.63 (1% low 79 fps) · worst 18.02 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 432 frames over 8.9 s, worst 2622.8 ms
- hitches (> 26.4 ms): **0** measured, 21 in warm-up
- timed GPU passes: 8.64 ms of a 10.63 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.54 ms, limit 11 ms
- pass worst frame 18.02 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.482 | 2.510 | 2.723 | 0.99 | 0.060 |
| dlss_super_resolution | 2.026 | 2.049 | 2.072 | 0.99 | 0.185 |
| early prepass | 0.834 | 0.843 | 1.041 | 0.99 | 0.089 |
| atmosphere_luts | 0.377 | 0.381 | 0.699 | 0.99 | 0.003 |
| bloom | 0.314 | 0.317 | 0.334 | 0.99 | 0.067 |
| main_transmissive_pass_3d | 0.269 | 0.272 | 0.295 | 0.99 | 0.009 |
| water_velocity | 0.241 | 0.243 | 0.269 | 0.99 | 0.002 |
| water_sspr | 0.219 | 0.221 | 0.225 | 0.99 | 0.002 |
| ssao | 0.204 | 0.206 | 0.339 | 0.99 | 0.008 |
| clouds_trace | 0.155 | 0.156 | 0.193 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.152 | 0.99 | 0.001 |
| uniform_allocation | 0.141 | 0.143 | 0.146 | 0.99 | 0.027 |
| water_fft | 0.139 | 0.140 | 0.142 | 0.99 | 0.003 |
| bin_unpacking | 0.115 | 0.116 | 0.115 | 0.99 | 0.020 |
| upscaling | 0.113 | 0.115 | 0.122 | 0.99 | 0.004 |
| clouds_composite | 0.113 | 0.114 | 0.116 | 0.99 | 0.002 |
| gpu_instance_materialization | 0.090 | 0.091 | 0.092 | 0.99 | 0.004 |
| early_mesh_preprocessing | 0.087 | 0.088 | 0.091 | 0.99 | 0.005 |
| tonemapping | 0.085 | 0.086 | 0.092 | 0.99 | 0.003 |
| clustering | 0.082 | 0.083 | 0.091 | 0.99 | 0.022 |
| render_sky | 0.054 | 0.054 | 0.055 | 0.99 | 0.002 |
| clouds_resolve | 0.051 | 0.052 | 0.054 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.050 | 0.418 | 0.419 | 0.12 | 0.001 |
| per_view_shadow_pass | 0.044 | 0.045 | 0.046 | 0.99 | 0.009 |
| water_snapshot | 0.033 | 0.033 | 0.035 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.032 | 0.036 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.029 | 0.99 | 0.001 |
| range_unpacking | 0.028 | 0.028 | 0.029 | 0.99 | 0.004 |
| lightprobe_radiance_map | 0.027 | 0.222 | 0.234 | 0.12 | 0.004 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.018 | 0.99 | 0.040 |
| night_sky | 0.004 | 0.004 | 0.009 | 0.99 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.721 | 10.882 | 17.15 | 13726.6 |
| encode//sum | 8.747 | 11.649 | 57.98 | 12351.0 |
| render/busy/ms | 8.336 | 9.549 | 16.51 | 11770.6 |
| main/busy/ms | 7.365 | 8.772 | 12.93 | 10399.8 |
| stage/render/render | 5.805 | 6.973 | 13.99 | 8196.7 |
| render/graph/ms | 5.082 | 6.184 | 13.10 | 7175.1 |
| pipelines/wait_ms | 4.372 | 0.000 | 2539.16 | 6172.6 |
| stage/main/post_update | 4.251 | 5.582 | 10.22 | 6002.8 |
| stage/render/graph_submit | 4.080 | 5.131 | 12.17 | 5760.4 |
| encode/finish_wall | 2.124 | 2.676 | 8.36 | 2999.0 |
| encode/submit | 1.982 | 2.686 | 7.65 | 2798.4 |
| stage/main/update | 1.859 | 2.258 | 2.76 | 2625.5 |
| stage/render/prepare_views | 0.938 | 2.054 | 3.25 | 1324.1 |
| stage/render/camera_driver | 0.936 | 1.112 | 1.44 | 1322.1 |
| render/acquire/ms | 0.892 | 2.011 | 3.20 | 1259.5 |
| stage/render/prepare_write_phase_buffers | 0.663 | 0.842 | 1.82 | 936.7 |
| render/present/ms | 0.493 | 0.586 | 4.44 | 696.5 |
| stage/render/prepare_assets | 0.450 | 0.580 | 1.71 | 635.3 |
| stage/render/prepare_resources | 0.394 | 0.465 | 2.26 | 556.0 |
| bigspace/total/ms | 0.390 | 0.831 | 2.68 | 550.8 |
| stage/render/prepare_flush | 0.325 | 0.529 | 1.13 | 458.7 |
| stage/main/pre_update | 0.319 | 0.411 | 0.61 | 450.2 |
| stage/main/first | 0.312 | 0.383 | 2.05 | 440.9 |
| stage/render/prepare_bind_groups | 0.261 | 0.356 | 0.61 | 368.2 |
| stage/render/prepare_batch_phases | 0.250 | 0.332 | 0.54 | 353.6 |
| stage/main/last | 0.240 | 0.299 | 0.78 | 338.6 |
| terrain/update/ms | 0.233 | 0.302 | 0.54 | 329.2 |
| terrain/select/ms | 0.202 | 0.260 | 0.50 | 285.3 |
| stream/drive/ms | 0.199 | 0.298 | 0.73 | 281.0 |
| stage/main/fixed_main_loop | 0.195 | 0.356 | 1.09 | 275.2 |
| stage/render/prepare_meshes | 0.176 | 0.232 | 0.49 | 249.2 |
| far/select/ms | 0.174 | 0.258 | 0.48 | 245.7 |
| stage/render/extract_commands | 0.170 | 0.207 | 0.45 | 240.4 |
| stream/tick/ms | 0.170 | 0.261 | 0.69 | 240.2 |
| controller/feed/ms | 0.165 | 0.220 | 0.45 | 232.6 |
| stage/render/create_views | 0.142 | 0.185 | 1.22 | 200.9 |
| bigspace/high_precision/ms | 0.134 | 0.315 | 2.46 | 189.6 |
| controller/move/ms | 0.129 | 0.267 | 0.59 | 181.5 |
| bigspace/low_precision/ms | 0.109 | 0.282 | 2.43 | 154.5 |
| pipelines/compile_ms | 0.106 | 0.000 | 119.43 | 149.3 |
| stage/render/queue | 0.103 | 0.136 | 0.27 | 145.2 |
| pipelines/front_ms | 0.103 | 0.000 | 116.33 | 144.8 |
| anim/evaluate/ms | 0.091 | 0.271 | 1.66 | 128.7 |
| stage/render/specialize | 0.082 | 0.110 | 0.33 | 116.2 |
| anim/evaluate_cpu/ms | 0.063 | 0.079 | 0.35 | 89.0 |
| stage/render/prepare_collect_phase_buffers | 0.050 | 0.079 | 0.20 | 71.3 |
| encode/resolve_diagnostics/sum | 0.045 | 0.079 | 0.41 | 63.7 |
| physics/step/ms | 0.031 | 0.060 | 0.33 | 44.0 |
| stage/render/cleanup | 0.026 | 0.036 | 0.31 | 36.4 |
| weather/map/ms | 0.024 | 0.033 | 0.11 | 33.3 |
| encode/camera_driver_span_begin/sum | 0.020 | 0.037 | 0.21 | 28.0 |
| encode/camera_driver_span_end/sum | 0.017 | 0.035 | 0.35 | 24.0 |
| stage/render/phase_sort | 0.012 | 0.025 | 0.19 | 17.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.15 | 15.1 |
| script/update/ms | 0.007 | 0.011 | 0.05 | 10.0 |
| controller/avoid/ms | 0.006 | 0.008 | 0.07 | 8.9 |
| script/ui/ms | 0.006 | 0.008 | 0.05 | 8.0 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 7.2 |
| anim/apply/ms | 0.005 | 0.006 | 0.01 | 6.4 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.10 | 4.5 |
| controller/write/ms | 0.002 | 0.003 | 0.03 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.39 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.8 |
| weather/apply/ms | 0.000 | 0.000 | 0.00 | 0.3 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4212 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.3 k | 37.3 k | 37.3 k |
| encode//max | ms | 1.80 | 8.02 | 1.68 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.21 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.35 | 0.02 |
| encode/finish_critical | ms | 1.80 | 8.02 | 1.68 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.41 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.40 | 16.51 | 9.35 |
| gpu/timed/ms | ms | 9.38 | 10.26 | 9.35 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1837 | 1837 | 1837 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 200.8 k | 200.8 k | 200.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 9431 | 9431 | 9431 |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 311.87 M | 311.87 M | 311.87 M |
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
| phases/shadow/batch_sets | count | 34 | 34 | 34 |
| phases/shadow/bins | count | 859 | 859 | 859 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 2848 | 2848 | 2848 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 27 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 165 | 165 | 165 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 7 | 7 | 7 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| preprocess/bind_groups_reused | count | 167 | 167 | 167 |
| process/cpu_usage | % | 30.92 | 42.58 | 29.19 |
| process/mem_usage | GiB | 4.29 | 4.39 | 4.39 |
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
| system/cpu_usage | % | 35.30 | 49.24 | 26.59 |
| system/mem_usage | % | 57.89 | 58.22 | 58.21 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2465 | 2465 | 2465 |
| textures/images | count | 1903 | 1903 | 1903 |
| views/all/batch_sets | count | 92 | 92 | 92 |
| views/all/bins | count | 3111 | 3111 | 3111 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 29.3 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 9108 | 9108 | 9108 |
| views/all/pipelines | count | 73 | 73 | 73 |
| views/all/pipelines_distinct | count | 61 | 61 | 61 |
| views/all/work_items | count | 255.0 k | 255.0 k | 255.0 k |
| views/cascade/batch_sets | count | 34 | 34 | 34 |
| views/cascade/bins | count | 859 | 859 | 859 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 2848 | 2848 | 2848 |
| views/cascade/pipelines | count | 27 | 27 | 27 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 145 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.51 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 32.25 | 34 | 45.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1412 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1412 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1412 |
| physics/inserts | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2622.8 | main/busy/ms 2592, stage/main/post_update 2492, stage/main/update 94.21, surface/advance_cpu_ms 67.25, stage/main/pre_update 2.74, stage/main/last 1.12, surface/cell_upload_ms 0.78, stage/main/first 0.58, stage/main/fixed_main_loop 0.42, stream/drive/ms 0.37, weather/map/ms 0.15, bigspace/total/ms 0.13 |
| warm-up | 3.57 | 951.7 | main/busy/ms 2628, stage/main/post_update 2518, render/thread/ms 910, render/busy/ms 783, stage/render/prepare_assets 515, stage/render/prepare_resources 238, stage/render/prepare_views 125, render/acquire/ms 124, stage/main/update 97.99, surface/advance_cpu_ms 67.25, stage/render/create_views 19.15, stage/render/render 9.51 |
| warm-up | 3.81 | 88.4 | main/busy/ms 59.89, render/thread/ms 49.95, stage/main/update 48.90, render/busy/ms 48.14, stage/render/prepare_assets 37.82, stage/main/post_update 7.51, stage/render/render 6.67, render/graph/ms 4.26, stage/render/graph_submit 2.28, stage/render/camera_driver 1.84, encode/submit 1.77, render/present/ms 1.57 |
| warm-up | 3.92 | 29.2 | main/busy/ms 72.30, stage/main/update 62.72, render/thread/ms 38.86, render/busy/ms 36.90, stage/render/prepare_assets 22.73, stage/render/render 8.22, stage/main/post_update 5.69, render/graph/ms 5.63, stage/render/graph_submit 3.42, encode/submit 2.86, stream/drive/ms 2.80, stage/render/camera_driver 2.00 |
| warm-up | 4.07 | 142.4 | render/thread/ms 95.62, main/busy/ms 72.39, render/busy/ms 67.02, stage/main/update 62.33, stage/render/prepare_assets 53.75, stage/render/render 34.79, render/present/ms 28.28, stage/main/post_update 6.42, render/graph/ms 5.60, stage/render/graph_submit 3.64, encode/submit 3.18, stage/render/camera_driver 1.80 |
| warm-up | 4.10 | 38.3 | pipelines/wait_ms 3940, pipelines/compile_ms 344, pipelines/front_ms 268, render/thread/ms 91.66, pipelines/driver_ms 75.29, render/busy/ms 63.89, stage/render/prepare_assets 56.15, stage/render/render 31.25, render/present/ms 27.61, main/busy/ms 15.59, stage/main/post_update 5.67, stage/main/update 5.35 |
| warm-up | 4.30 | 99.8 | pipelines/wait_ms 4101, pipelines/compile_ms 153, pipelines/front_ms 143, main/busy/ms 89.48, stage/main/update 82.42, render/thread/ms 34.62, render/busy/ms 33.32, stage/render/prepare_assets 22.90, pipelines/driver_ms 9.68, stage/render/render 5.55, stage/main/post_update 4.25, render/graph/ms 3.92 |
| warm-up | 4.40 | 94.8 | main/busy/ms 73.20, stage/main/update 53.13, render/thread/ms 46.41, render/busy/ms 44.54, stream/drive/ms 40.78, stage/render/prepare_assets 29.92, stage/main/post_update 13.12, stage/render/render 8.37, render/graph/ms 5.90, encode//sum 3.91, stage/render/graph_submit 3.89, encode/submit 3.22 |
| warm-up | 4.44 | 39.2 | render/thread/ms 61.49, render/busy/ms 59.44, stage/render/prepare_assets 46.01, main/busy/ms 36.04, stage/main/update 16.33, stage/main/post_update 12.23, stage/render/render 7.64, stream/drive/ms 5.62, render/graph/ms 5.19, stage/render/graph_submit 3.44, stage/main/fixed_main_loop 3.01, encode/submit 2.86 |
| warm-up | 4.50 | 52.2 | render/thread/ms 66.30, render/busy/ms 36.65, stage/render/render 32.64, render/present/ms 29.50, stage/render/prepare_assets 28.04, main/busy/ms 22.90, stage/main/update 10.12, stage/main/post_update 5.19, stream/drive/ms 4.82, stage/main/fixed_main_loop 2.93, render/graph/ms 2.68, stage/main/pre_update 1.88 |
| warm-up | 4.63 | 36.4 | render/thread/ms 38.79, render/busy/ms 37.13, main/busy/ms 34.61, stage/render/prepare_assets 20.27, stage/main/update 19.71, stage/main/post_update 9.03, stage/render/render 7.78, render/graph/ms 5.49, stream/drive/ms 5.48, stage/render/graph_submit 3.58, encode/submit 2.83, encode//sum 2.71 |
| warm-up | 4.67 | 31.8 | pipelines/wait_ms 1061, render/thread/ms 40.87, render/busy/ms 39.69, stage/render/prepare_assets 27.39, main/busy/ms 27.04, stage/main/update 12.26, stage/main/post_update 7.61, stream/drive/ms 6.27, stage/render/render 4.67, pipelines/compile_ms 3.56, pipelines/front_ms 3.25, render/graph/ms 3.25 |
| warm-up | 4.95 | 32.4 | pipelines/wait_ms 7480, pipelines/compile_ms 150, pipelines/front_ms 140, render/thread/ms 40.95, render/busy/ms 39.21, main/busy/ms 32.65, stage/render/prepare_assets 16.85, stage/main/update 16.50, pipelines/driver_ms 10.75, stage/render/render 9.90, stage/main/post_update 9.66, render/graph/ms 7.52 |
| warm-up | 4.97 | 29.5 | pipelines/wait_ms 2620, render/thread/ms 49.82, render/busy/ms 47.73, main/busy/ms 32.84, stage/render/prepare_assets 20.64, pipelines/compile_ms 16.40, pipelines/front_ms 15.29, stage/main/update 14.73, stage/main/post_update 12.46, stage/render/render 11.44, render/graph/ms 8.31, stage/render/graph_submit 5.23 |
| warm-up | 5.01 | 35.6 | pipelines/wait_ms 6645, pipelines/compile_ms 84.24, pipelines/front_ms 80.42, render/thread/ms 44.08, render/busy/ms 42.44, main/busy/ms 22.69, stage/render/prepare_assets 21.91, stage/main/post_update 8.77, stage/main/update 8.65, stage/render/render 7.50, render/graph/ms 5.11, pipelines/driver_ms 3.82 |
| warm-up | 5.18 | 28.3 | pipelines/wait_ms 8515, pipelines/compile_ms 36.87, pipelines/front_ms 34.09, main/busy/ms 25.96, render/thread/ms 19.54, render/busy/ms 18.41, stage/main/update 16.96, stream/drive/ms 12.57, stage/render/render 7.76, render/graph/ms 6.34, stage/main/post_update 6.11, stage/render/graph_submit 4.75 |
| warm-up | 5.29 | 31.6 | main/busy/ms 30.45, render/thread/ms 29.76, render/busy/ms 28.69, stage/main/update 21.01, stage/render/prepare_assets 13.22, stream/drive/ms 6.85, stage/render/render 6.24, stage/main/post_update 5.49, render/graph/ms 4.79, encode//sum 4.47, stage/render/graph_submit 3.03, stage/render/prepare_meshes 2.92 |
| warm-up | 5.63 | 28.8 | pipelines/wait_ms 1016, pipelines/compile_ms 144, pipelines/front_ms 140, render/thread/ms 35.42, render/busy/ms 33.85, main/busy/ms 24.56, stage/render/prepare_assets 17.17, stage/main/update 11.45, stage/main/post_update 8.05, stage/render/render 6.99, encode//sum 5.94, render/graph/ms 4.78 |
| warm-up | 6.41 | 31.9 | main/busy/ms 41.01, render/thread/ms 37.38, render/busy/ms 35.90, stage/main/update 28.25, stage/render/prepare_assets 13.86, stage/main/post_update 8.49, stage/render/render 8.15, encode//sum 7.83, render/graph/ms 6.19, stage/render/prepare_meshes 4.99, stage/render/graph_submit 4.16, encode/submit 2.56 |
| warm-up | 6.49 | 39.7 | main/busy/ms 42.74, render/thread/ms 41.77, render/busy/ms 40.11, stage/main/update 30.28, stage/render/prepare_assets 16.78, stage/render/render 10.47, encode//sum 10.03, render/graph/ms 8.20, stage/main/post_update 8.19, stage/render/graph_submit 5.46, stage/render/prepare_bind_groups 3.65, encode/submit 3.52 |
| warm-up | 6.86 | 31.1 | main/busy/ms 35.88, render/thread/ms 29.47, render/busy/ms 28.28, stage/main/update 23.31, encode//sum 12.87, stage/render/render 10.88, stage/render/prepare_assets 9.17, render/graph/ms 9.08, stage/main/post_update 8.62, stage/render/graph_submit 6.30, encode/submit 3.97, stage/render/camera_driver 2.58 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on15\balmora@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 4% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **97%** (min 86%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.1 W · 75 °C (max 78) · VRAM 5518 MiB · throttled 100% of samples
