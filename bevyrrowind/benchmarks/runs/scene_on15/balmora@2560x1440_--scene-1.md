# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1414 frames over 15.0 s · **median 10.52 ms (95 fps)** · mean 10.61 · p95 11.89 · p99 12.73 (1% low 79 fps) · worst 17.17 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 336 frames over 8.0 s, worst 2652.8 ms
- hitches (> 26.3 ms): **0** measured, 19 in warm-up
- timed GPU passes: 8.52 ms of a 10.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.52 ms, limit 11 ms
- pass worst frame 17.17 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.462 | 2.526 | 2.710 | 0.97 | 0.069 |
| dlss_super_resolution | 1.999 | 2.052 | 2.075 | 0.97 | 0.185 |
| early prepass | 0.800 | 0.820 | 1.009 | 0.97 | 0.090 |
| atmosphere_luts | 0.375 | 0.385 | 0.677 | 0.97 | 0.004 |
| bloom | 0.310 | 0.319 | 0.334 | 0.97 | 0.066 |
| main_transmissive_pass_3d | 0.269 | 0.276 | 0.294 | 0.97 | 0.009 |
| water_velocity | 0.239 | 0.245 | 0.270 | 0.97 | 0.002 |
| water_sspr | 0.214 | 0.220 | 0.223 | 0.97 | 0.002 |
| ssao | 0.205 | 0.210 | 0.341 | 0.97 | 0.008 |
| clouds_trace | 0.148 | 0.152 | 0.187 | 0.97 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.148 | 0.152 | 0.97 | 0.002 |
| uniform_allocation | 0.143 | 0.147 | 0.146 | 0.97 | 0.033 |
| water_fft | 0.139 | 0.143 | 0.144 | 0.97 | 0.003 |
| clouds_composite | 0.111 | 0.114 | 0.116 | 0.97 | 0.002 |
| upscaling | 0.111 | 0.113 | 0.122 | 0.97 | 0.004 |
| bin_unpacking | 0.110 | 0.113 | 0.113 | 0.97 | 0.022 |
| gpu_instance_materialization | 0.088 | 0.091 | 0.092 | 0.97 | 0.004 |
| early_mesh_preprocessing | 0.088 | 0.090 | 0.096 | 0.97 | 0.006 |
| tonemapping | 0.085 | 0.087 | 0.092 | 0.97 | 0.004 |
| clustering | 0.075 | 0.077 | 0.086 | 0.97 | 0.023 |
| render_sky | 0.053 | 0.054 | 0.055 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.426 | 0.422 | 0.12 | 0.001 |
| clouds_resolve | 0.051 | 0.052 | 0.054 | 0.97 | 0.001 |
| per_view_shadow_pass | 0.044 | 0.045 | 0.045 | 0.97 | 0.009 |
| water_snapshot | 0.031 | 0.032 | 0.034 | 0.97 | 0.002 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.028 | 0.029 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.222 | 0.233 | 0.12 | 0.005 |
| range_unpacking | 0.026 | 0.026 | 0.028 | 0.97 | 0.005 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.97 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.97 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.97 | 0.003 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.018 | 0.97 | 0.042 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.97 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.007 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 33.855 | 0.000 | 4559.47 | 47870.6 |
| render/thread/ms | 9.690 | 10.934 | 16.31 | 13701.4 |
| encode//sum | 9.146 | 11.909 | 32.70 | 12932.6 |
| render/busy/ms | 8.599 | 9.861 | 15.79 | 12159.2 |
| main/busy/ms | 7.591 | 9.074 | 14.78 | 10733.2 |
| stage/render/render | 5.973 | 7.263 | 13.16 | 8445.9 |
| render/graph/ms | 5.223 | 6.377 | 12.46 | 7385.5 |
| stage/main/post_update | 4.356 | 5.770 | 11.57 | 6159.3 |
| stage/render/graph_submit | 4.186 | 5.275 | 11.39 | 5919.3 |
| encode/finish_wall | 2.247 | 2.846 | 8.57 | 3176.9 |
| encode/submit | 1.985 | 2.781 | 4.40 | 2806.5 |
| stage/main/update | 1.956 | 2.327 | 3.16 | 2765.3 |
| stage/render/camera_driver | 0.971 | 1.153 | 1.42 | 1373.4 |
| pipelines/compile_ms | 0.844 | 0.000 | 131.29 | 1193.9 |
| pipelines/front_ms | 0.817 | 0.000 | 128.02 | 1155.8 |
| stage/render/prepare_write_phase_buffers | 0.689 | 0.867 | 1.24 | 974.3 |
| stage/render/prepare_views | 0.619 | 1.689 | 3.03 | 875.6 |
| render/acquire/ms | 0.573 | 1.646 | 2.98 | 809.7 |
| render/present/ms | 0.518 | 0.645 | 2.92 | 732.6 |
| stage/render/prepare_assets | 0.469 | 0.624 | 1.62 | 663.0 |
| stage/render/prepare_resources | 0.411 | 0.498 | 0.86 | 580.9 |
| bigspace/total/ms | 0.409 | 1.006 | 3.05 | 578.9 |
| stage/main/pre_update | 0.335 | 0.442 | 0.83 | 473.5 |
| stage/render/prepare_flush | 0.321 | 0.507 | 0.96 | 453.4 |
| stage/main/first | 0.313 | 0.392 | 1.42 | 443.0 |
| stage/render/prepare_bind_groups | 0.273 | 0.367 | 0.80 | 386.5 |
| stage/render/prepare_batch_phases | 0.255 | 0.349 | 0.62 | 360.9 |
| terrain/update/ms | 0.240 | 0.322 | 0.57 | 339.9 |
| stage/main/last | 0.237 | 0.289 | 0.47 | 335.8 |
| stream/drive/ms | 0.217 | 0.331 | 0.62 | 306.1 |
| terrain/select/ms | 0.208 | 0.278 | 0.54 | 294.7 |
| stage/main/fixed_main_loop | 0.208 | 0.391 | 1.01 | 294.6 |
| stage/render/prepare_meshes | 0.192 | 0.251 | 0.51 | 271.0 |
| stream/tick/ms | 0.185 | 0.290 | 0.59 | 261.6 |
| far/select/ms | 0.184 | 0.276 | 0.59 | 260.6 |
| stage/render/extract_commands | 0.184 | 0.224 | 0.39 | 259.5 |
| controller/feed/ms | 0.177 | 0.235 | 0.44 | 250.2 |
| bigspace/high_precision/ms | 0.153 | 0.386 | 2.57 | 216.3 |
| stage/render/create_views | 0.149 | 0.195 | 0.45 | 210.5 |
| controller/move/ms | 0.122 | 0.199 | 0.52 | 172.8 |
| bigspace/low_precision/ms | 0.112 | 0.243 | 2.85 | 158.8 |
| stage/render/queue | 0.107 | 0.141 | 0.39 | 151.3 |
| stage/render/specialize | 0.089 | 0.120 | 0.27 | 126.1 |
| anim/evaluate/ms | 0.073 | 0.215 | 2.42 | 102.5 |
| anim/evaluate_cpu/ms | 0.065 | 0.082 | 0.35 | 91.9 |
| stage/render/prepare_collect_phase_buffers | 0.050 | 0.076 | 0.33 | 70.4 |
| encode/resolve_diagnostics/sum | 0.049 | 0.086 | 0.50 | 69.3 |
| physics/step/ms | 0.035 | 0.066 | 0.72 | 48.9 |
| pipelines/driver_ms | 0.027 | 0.000 | 4.45 | 38.1 |
| weather/map/ms | 0.026 | 0.035 | 0.34 | 36.1 |
| stage/render/cleanup | 0.025 | 0.035 | 0.22 | 34.8 |
| encode/camera_driver_span_begin/sum | 0.021 | 0.038 | 0.10 | 29.4 |
| encode/camera_driver_span_end/sum | 0.019 | 0.035 | 0.37 | 26.2 |
| stage/render/phase_sort | 0.012 | 0.025 | 0.19 | 17.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.10 | 16.2 |
| physics/insert/ms | 0.009 | 0.041 | 1.04 | 12.2 |
| script/update/ms | 0.008 | 0.013 | 0.05 | 11.7 |
| script/input/ms | 0.006 | 0.007 | 0.13 | 8.2 |
| controller/avoid/ms | 0.006 | 0.008 | 0.10 | 8.2 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 7.0 |
| anim/apply/ms | 0.005 | 0.006 | 0.13 | 6.6 |
| controller/write/ms | 0.002 | 0.002 | 0.04 | 2.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.04 | 0.4 |
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
| encode//max | ms | 1.89 | 8.50 | 2.12 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.10 | 0.05 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.37 | 0.04 |
| encode/finish_critical | ms | 1.89 | 8.50 | 2.12 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.50 | 0.15 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.39 | 15.79 | 9.56 |
| gpu/timed/ms | ms | 9.37 | 10.53 | 9.56 |
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
| physics/pending | count | 0 | 611 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 165 | 165 | 165 |
| pipelines/waiting | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 7 | 15 | 7 |
| preprocess/bind_groups_created | count | 0 | 8 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| preprocess/bind_groups_reused | count | 167 | 167 | 167 |
| process/cpu_usage | % | 28.33 | 43.00 | 33.17 |
| process/mem_usage | GiB | 4.31 | 4.39 | 4.39 |
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
| system/cpu_usage | % | 36.47 | 45.72 | 41.13 |
| system/mem_usage | % | 57.88 | 58.39 | 58.15 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.11 | 3 | 155 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.50 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 32.25 | 34 | 45.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1414 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1414 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1414 |
| physics/inserts | count | 0.38 | 173 | 540 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.3 ms; slowest 19, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.65 | 2652.8 | main/busy/ms 2627, stage/main/post_update 2529, stage/main/update 92.82, surface/advance_cpu_ms 67.05, stage/main/pre_update 2.47, stage/main/last 1.12, surface/cell_upload_ms 0.76, stage/main/first 0.57, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.39, weather/map/ms 0.15, bigspace/total/ms 0.12 |
| warm-up | 3.42 | 767.2 | main/busy/ms 2660, stage/main/post_update 2552, render/thread/ms 727, render/busy/ms 628, stage/render/prepare_assets 380, stage/render/prepare_resources 229, stage/render/prepare_views 97.59, render/acquire/ms 97.39, stage/main/update 96.43, surface/advance_cpu_ms 67.05, stage/render/render 8.10, stage/render/create_views 8.09 |
| warm-up | 3.67 | 90.1 | main/busy/ms 58.33, render/thread/ms 54.26, render/busy/ms 52.22, stage/main/update 47.87, stage/render/prepare_assets 40.01, stage/render/render 6.80, stage/main/post_update 6.78, render/graph/ms 4.28, stage/render/graph_submit 2.41, encode/submit 1.92, stage/render/camera_driver 1.72, stream/drive/ms 1.64 |
| warm-up | 3.87 | 116.7 | main/busy/ms 82.35, render/thread/ms 74.04, render/busy/ms 71.07, stage/main/update 69.06, stage/render/prepare_assets 55.16, stage/render/render 11.87, render/graph/ms 8.25, stage/main/post_update 7.67, stage/render/graph_submit 5.75, encode/submit 5.63, render/present/ms 2.69, stage/main/pre_update 2.58 |
| warm-up | 3.96 | 89.4 | pipelines/wait_ms 1207, pipelines/compile_ms 241, pipelines/front_ms 233, render/thread/ms 119, render/busy/ms 90.23, stage/render/prepare_assets 77.09, main/busy/ms 47.21, stage/main/update 36.80, stage/render/render 36.02, render/present/ms 28.69, pipelines/driver_ms 7.77, render/graph/ms 6.59 |
| warm-up | 4.01 | 43.0 | pipelines/wait_ms 4185, pipelines/compile_ms 321, pipelines/front_ms 257, render/thread/ms 85.68, pipelines/driver_ms 64.00, render/busy/ms 58.14, stage/render/prepare_assets 45.68, stage/render/render 36.04, render/present/ms 27.35, main/busy/ms 18.09, stage/main/post_update 8.43, render/graph/ms 8.12 |
| warm-up | 4.19 | 136.7 | pipelines/wait_ms 345, pipelines/compile_ms 143, pipelines/front_ms 140, main/busy/ms 115, stage/main/update 108, render/thread/ms 37.37, render/busy/ms 35.97, stage/render/prepare_assets 26.70, stream/drive/ms 15.42, stage/render/render 6.41, render/graph/ms 4.68, stage/main/post_update 4.39 |
| warm-up | 4.29 | 87.6 | main/busy/ms 65.36, stage/main/update 45.42, render/thread/ms 43.69, render/busy/ms 41.77, stream/drive/ms 36.32, stage/render/prepare_assets 25.45, stage/main/post_update 12.04, stage/render/render 9.61, render/graph/ms 6.92, stage/render/graph_submit 4.86, encode/submit 3.99, stage/main/pre_update 3.06 |
| warm-up | 4.32 | 31.7 | render/thread/ms 55.23, render/busy/ms 53.87, stage/render/prepare_assets 39.17, main/busy/ms 38.81, stage/main/update 20.28, stream/drive/ms 12.11, stage/main/post_update 10.97, stage/render/render 6.84, render/graph/ms 4.76, stage/render/graph_submit 2.99, stage/main/pre_update 2.95, encode/submit 2.34 |
| warm-up | 4.40 | 28.0 | pipelines/wait_ms 697, pipelines/compile_ms 231, pipelines/front_ms 219, render/thread/ms 41.89, render/busy/ms 39.13, main/busy/ms 29.15, stage/render/prepare_assets 18.14, stage/main/update 14.77, pipelines/driver_ms 12.43, stage/render/render 11.09, stage/main/post_update 10.10, render/graph/ms 7.31 |
| warm-up | 4.43 | 37.1 | pipelines/wait_ms 697, pipelines/compile_ms 231, pipelines/front_ms 219, render/thread/ms 44.41, render/busy/ms 42.28, main/busy/ms 30.17, stage/render/prepare_assets 27.36, stage/main/update 16.41, pipelines/driver_ms 12.43, stage/main/post_update 9.30, stage/render/render 8.83, stream/drive/ms 8.75 |
| warm-up | 4.47 | 30.9 | pipelines/wait_ms 2921, pipelines/compile_ms 93.10, pipelines/front_ms 90.40, render/thread/ms 40.78, render/busy/ms 39.63, stage/render/prepare_assets 31.27, main/busy/ms 25.69, stage/main/update 14.43, stream/drive/ms 9.92, stage/main/post_update 7.57, stage/render/render 5.35, render/graph/ms 3.87 |
| warm-up | 4.71 | 28.0 | pipelines/wait_ms 2110, pipelines/compile_ms 152, pipelines/front_ms 143, render/thread/ms 35.01, render/busy/ms 33.07, main/busy/ms 23.42, stage/render/prepare_assets 17.50, stage/main/update 12.39, pipelines/driver_ms 9.61, stage/render/render 7.19, stage/main/post_update 6.70, render/graph/ms 4.70 |
| warm-up | 4.83 | 35.2 | render/thread/ms 44.34, render/busy/ms 42.55, main/busy/ms 29.13, stage/render/prepare_assets 23.87, stage/main/update 12.18, stage/main/post_update 12.03, stage/render/render 7.45, render/graph/ms 5.13, stream/drive/ms 4.13, encode//sum 3.51, stage/render/graph_submit 2.85, stage/render/prepare_write_phase_buffers 2.69 |
| warm-up | 5.94 | 28.5 | pipelines/wait_ms 9192, pipelines/compile_ms 42.09, pipelines/front_ms 38.91, render/thread/ms 31.91, render/busy/ms 30.48, main/busy/ms 27.14, stage/main/update 17.42, stage/render/prepare_assets 9.70, stage/render/render 9.43, encode//sum 7.68, render/graph/ms 7.25, stage/main/post_update 6.25 |
| warm-up | 5.98 | 32.6 | main/busy/ms 42.88, stage/main/update 33.48, render/thread/ms 29.83, render/busy/ms 28.10, stage/render/render 11.52, encode//sum 10.50, render/graph/ms 9.07, stage/render/prepare_assets 7.58, stage/main/post_update 6.42, stage/render/graph_submit 5.67, encode/submit 3.31, stage/render/camera_driver 3.20 |
| warm-up | 6.01 | 27.5 | render/thread/ms 34.33, render/busy/ms 33.11, main/busy/ms 29.08, stage/main/update 18.92, stage/render/prepare_assets 17.62, encode//sum 8.73, stage/render/render 7.99, stage/main/post_update 6.69, render/graph/ms 6.20, stage/render/graph_submit 4.16, encode/submit 2.39, stage/render/camera_driver 1.90 |
| warm-up | 6.13 | 47.8 | pipelines/wait_ms 12.5 k, main/busy/ms 62.01, pipelines/compile_ms 59.52, pipelines/front_ms 55.48, stage/main/update 43.42, render/thread/ms 41.11, render/busy/ms 39.17, stage/main/post_update 15.18, encode//sum 14.29, stage/render/render 13.47, render/graph/ms 10.64, stage/render/prepare_assets 10.14 |
| warm-up | 6.19 | 58.9 | pipelines/wait_ms 19.8 k, pipelines/compile_ms 96.38, pipelines/front_ms 90.31, main/busy/ms 66.16, render/thread/ms 62.22, render/busy/ms 60.25, stage/main/update 49.35, stage/render/prepare_assets 30.32, encode//sum 13.64, stage/render/render 13.23, stage/main/post_update 11.79, render/graph/ms 10.68 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on15\balmora@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 20%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 91%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.0 W · 76 °C (max 78) · VRAM 5764 MiB · throttled 100% of samples
