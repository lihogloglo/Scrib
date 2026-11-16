# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1486 frames over 15.0 s · **median 10.01 ms (100 fps)** · mean 10.10 · p95 11.44 · p99 12.26 (1% low 82 fps) · worst 24.19 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 310 frames over 8.5 s, worst 2509.3 ms
- hitches (> 25.0 ms): **0** measured, 41 in warm-up
- timed GPU passes: 8.75 ms of a 10.10 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.01 ms, limit 11 ms
- pass worst frame 24.19 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.515 | 2.548 | 2.626 | 0.99 | 0.066 |
| dlss_super_resolution | 2.031 | 2.057 | 2.080 | 0.99 | 0.180 |
| early prepass | 0.854 | 0.865 | 1.041 | 0.99 | 0.090 |
| atmosphere_luts | 0.391 | 0.396 | 0.691 | 0.99 | 0.003 |
| bloom | 0.316 | 0.320 | 0.336 | 0.99 | 0.063 |
| main_transmissive_pass_3d | 0.277 | 0.281 | 0.298 | 0.99 | 0.009 |
| water_velocity | 0.243 | 0.246 | 0.270 | 0.99 | 0.001 |
| water_sspr | 0.210 | 0.213 | 0.215 | 0.99 | 0.002 |
| ssao | 0.201 | 0.204 | 0.212 | 0.99 | 0.007 |
| per_view_shadow_pass_cascade1 | 0.172 | 0.699 | 0.951 | 0.25 | 0.048 |
| atmos_froxel_inject | 0.150 | 0.152 | 0.160 | 0.99 | 0.001 |
| clouds_trace | 0.149 | 0.151 | 0.179 | 0.99 | 0.001 |
| water_fft | 0.134 | 0.135 | 0.139 | 0.99 | 0.003 |
| uniform_allocation | 0.119 | 0.121 | 0.139 | 0.99 | 0.021 |
| upscaling | 0.114 | 0.116 | 0.125 | 0.99 | 0.004 |
| clouds_composite | 0.113 | 0.114 | 0.116 | 0.99 | 0.002 |
| bin_unpacking | 0.098 | 0.099 | 0.113 | 0.99 | 0.016 |
| tonemapping | 0.085 | 0.087 | 0.090 | 0.99 | 0.004 |
| gpu_instance_materialization | 0.075 | 0.076 | 0.093 | 0.99 | 0.004 |
| clustering | 0.069 | 0.070 | 0.078 | 0.99 | 0.021 |
| early_mesh_preprocessing | 0.061 | 0.062 | 0.080 | 0.99 | 0.004 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.425 | 0.420 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.032 | 0.036 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.223 | 0.234 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.029 | 0.99 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 0.99 | 0.002 |
| range_unpacking | 0.023 | 0.023 | 0.033 | 0.99 | 0.003 |
| water_mips | 0.022 | 0.023 | 0.023 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.027 | 0.99 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.014 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.99 | 0.003 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.017 | 0.99 | 0.041 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.99 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.016 | 0.016 | 0.00 | 0.005 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| per_view_shadow_pass_cascade3 | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.210 | 10.535 | 23.31 | 13686.8 |
| encode//sum | 8.009 | 10.736 | 63.66 | 11900.9 |
| render/busy/ms | 7.825 | 8.916 | 22.66 | 11628.2 |
| main/busy/ms | 7.209 | 8.474 | 9.91 | 10712.1 |
| stage/render/render | 5.498 | 6.462 | 20.06 | 8170.4 |
| render/graph/ms | 4.797 | 5.709 | 19.24 | 7127.6 |
| stage/main/post_update | 4.147 | 5.330 | 6.69 | 6162.9 |
| stage/render/graph_submit | 3.830 | 4.687 | 18.29 | 5690.8 |
| encode/submit | 1.945 | 2.482 | 4.73 | 2890.1 |
| encode/finish_wall | 1.916 | 2.410 | 14.56 | 2846.7 |
| stage/main/update | 1.849 | 2.205 | 2.65 | 2747.2 |
| stage/render/prepare_views | 0.945 | 2.428 | 3.19 | 1404.7 |
| stage/render/camera_driver | 0.905 | 1.053 | 1.55 | 1345.4 |
| render/acquire/ms | 0.902 | 2.380 | 3.14 | 1340.5 |
| stage/render/prepare_write_phase_buffers | 0.558 | 0.719 | 1.06 | 829.8 |
| render/present/ms | 0.483 | 0.568 | 1.79 | 718.1 |
| stage/render/prepare_assets | 0.439 | 0.559 | 1.67 | 651.9 |
| stage/render/prepare_resources | 0.387 | 0.461 | 0.81 | 574.9 |
| bigspace/total/ms | 0.385 | 0.957 | 2.63 | 571.4 |
| stage/main/pre_update | 0.312 | 0.398 | 0.68 | 463.2 |
| stage/main/first | 0.298 | 0.363 | 1.33 | 442.8 |
| stage/render/prepare_flush | 0.288 | 0.489 | 0.81 | 428.4 |
| stage/render/prepare_bind_groups | 0.257 | 0.352 | 0.62 | 382.3 |
| stage/main/last | 0.243 | 0.304 | 0.64 | 360.9 |
| terrain/update/ms | 0.235 | 0.305 | 0.53 | 349.4 |
| terrain/select/ms | 0.205 | 0.264 | 0.50 | 303.9 |
| stream/drive/ms | 0.199 | 0.292 | 0.51 | 295.0 |
| stage/render/prepare_batch_phases | 0.195 | 0.271 | 0.38 | 289.1 |
| stage/main/fixed_main_loop | 0.182 | 0.334 | 1.10 | 270.9 |
| stage/render/prepare_meshes | 0.180 | 0.232 | 0.44 | 267.6 |
| stream/tick/ms | 0.171 | 0.254 | 0.47 | 253.7 |
| stage/render/extract_commands | 0.170 | 0.205 | 0.37 | 252.2 |
| far/select/ms | 0.168 | 0.245 | 0.49 | 250.1 |
| controller/feed/ms | 0.167 | 0.220 | 0.73 | 247.9 |
| bigspace/high_precision/ms | 0.152 | 0.542 | 2.36 | 225.7 |
| stage/render/create_views | 0.143 | 0.184 | 0.58 | 212.5 |
| controller/move/ms | 0.129 | 0.249 | 0.54 | 191.2 |
| bigspace/low_precision/ms | 0.107 | 0.296 | 2.21 | 158.9 |
| anim/evaluate/ms | 0.102 | 0.246 | 1.88 | 152.2 |
| stage/render/queue | 0.098 | 0.129 | 0.32 | 144.9 |
| stage/render/specialize | 0.087 | 0.116 | 0.34 | 129.6 |
| anim/evaluate_cpu/ms | 0.060 | 0.076 | 0.42 | 89.5 |
| encode/resolve_diagnostics/sum | 0.053 | 0.082 | 0.16 | 78.0 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.076 | 0.29 | 70.6 |
| physics/step/ms | 0.029 | 0.056 | 0.29 | 42.5 |
| weather/map/ms | 0.025 | 0.034 | 0.26 | 37.6 |
| stage/render/cleanup | 0.024 | 0.033 | 0.48 | 35.9 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.037 | 0.20 | 35.1 |
| encode/camera_driver_span_end/sum | 0.020 | 0.035 | 0.08 | 30.2 |
| stage/render/phase_sort | 0.014 | 0.026 | 0.15 | 20.9 |
| weather/eval/ms | 0.012 | 0.016 | 0.10 | 17.3 |
| script/update/ms | 0.007 | 0.010 | 0.20 | 10.1 |
| controller/avoid/ms | 0.006 | 0.008 | 0.24 | 9.4 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 7.4 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 7.3 |
| anim/apply/ms | 0.004 | 0.005 | 0.27 | 6.4 |
| controller/write/ms | 0.002 | 0.003 | 0.05 | 3.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.11 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.3 k | 37.3 k | 37.3 k |
| encode//max | ms | 1.58 | 14.48 | 1.71 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.20 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.08 | 0.02 |
| encode/finish_critical | ms | 1.58 | 14.48 | 1.71 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.16 | 0.09 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.97 | 22.66 | 10.28 |
| gpu/timed/ms | ms | 9.97 | 11.86 | 10.28 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1929 | 1929 | 1929 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 214.7 k | 214.7 k | 214.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.8 k | 10.8 k | 10.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
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
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 877 | 877 | 877 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 877 | 877 | 877 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 5 | 24 | 5 |
| phases/shadow/bins | count | 857 | 870 | 857 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 5861 | 11.7 k | 5861 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 3121 | 3149 | 3121 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 20 | 23 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 294 | 294 | 294 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 5 | 37 | 5 |
| preprocess/bind_groups_created | count | 0 | 31 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 127 | 127 | 127 |
| process/cpu_usage | % | 26.39 | 38.84 | 23.02 |
| process/mem_usage | GiB | 4.61 | 4.71 | 4.71 |
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
| started_load_count |  | 5285 | 5287 | 5287 |
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
| system/cpu_usage | % | 32.30 | 49.79 | 27.64 |
| system/mem_usage | % | 47.96 | 48.27 | 48.27 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2538 | 2538 | 2538 |
| textures/images | count | 1995 | 1995 | 1995 |
| views/all/batch_sets | count | 65 | 84 | 65 |
| views/all/bins | count | 3109 | 3122 | 3109 |
| views/all/count | count | 3 | 5 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 17.6 k | 23.4 k | 17.6 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 9381 | 9409 | 9381 |
| views/all/pipelines | count | 68 | 71 | 68 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.8 k | 206.1 k | 153.8 k |
| views/cascade/batch_sets | count | 5 | 24 | 5 |
| views/cascade/bins | count | 857 | 870 | 857 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 5861 | 11.7 k | 5861 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 3121 | 3149 | 3121 |
| views/cascade/pipelines | count | 20 | 23 | 20 |
| views/cascade/work_items | count | 49.2 k | 101.4 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 60 | 60 | 60 |
| views/main/bins | count | 2252 | 2252 | 2252 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6260 | 6260 | 6260 |
| views/main/pipelines | count | 48 | 48 | 48 |
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
| anim/events/count | count | 0.10 | 3 | 150 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.33 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 32.50 | 35 | 48.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1486 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1486 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1486 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.61 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
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
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2509.3 | main/busy/ms 2482, stage/main/post_update 2381, stage/main/update 95.24, surface/advance_cpu_ms 67.23, stage/main/pre_update 2.77, stage/main/last 1.05, surface/cell_upload_ms 0.96, stream/drive/ms 0.61, stage/main/first 0.48, stage/main/fixed_main_loop 0.41, stream/tick/ms 0.12, weather/map/ms 0.12 |
| warm-up | 3.24 | 735.1 | main/busy/ms 2611, stage/main/post_update 2404, render/thread/ms 704, render/busy/ms 537, stage/render/prepare_assets 502, stage/main/update 195, stage/render/prepare_views 165, render/acquire/ms 164, surface/advance_cpu_ms 67.23, stage/render/prepare_resources 20.83, stage/render/render 10.13, encode//sum 8.85 |
| warm-up | 3.33 | 90.3 | render/thread/ms 777, render/busy/ms 609, stage/render/prepare_assets 550, stage/render/prepare_views 165, render/acquire/ms 164, main/busy/ms 153, stage/main/update 105, stage/main/post_update 37.02, stage/render/prepare_resources 26.65, stage/render/render 24.35, pipelines/compile_ms 22.27, render/graph/ms 18.92 |
| warm-up | 3.40 | 62.5 | pipelines/compile_ms 438, pipelines/front_ms 404, pipelines/wait_ms 315, render/thread/ms 134, render/busy/ms 131, main/busy/ms 84.70, stage/render/render 69.44, stage/main/post_update 67.17, render/graph/ms 65.97, stage/render/prepare_assets 48.33, stage/render/graph_submit 39.35, pipelines/driver_ms 34.41 |
| warm-up | 3.54 | 139.4 | pipelines/wait_ms 4924, pipelines/compile_ms 1629, pipelines/front_ms 1541, render/thread/ms 199, render/busy/ms 197, main/busy/ms 154, stage/main/post_update 137, pipelines/driver_ms 88.43, stage/render/render 65.11, render/graph/ms 62.29, stage/render/prepare_resources 59.66, stage/render/graph_submit 36.07 |
| warm-up | 3.57 | 30.3 | pipelines/wait_ms 5319, pipelines/compile_ms 1835, pipelines/front_ms 1742, main/busy/ms 169, render/thread/ms 167, render/busy/ms 165, pipelines/driver_ms 93.61, stage/main/post_update 92.92, stage/render/prepare_resources 62.38, stage/main/update 58.34, stage/render/prepare_flush 35.31, stage/render/prepare_assets 34.17 |
| warm-up | 3.73 | 163.5 | pipelines/compile_ms 2389, pipelines/front_ms 2291, pipelines/wait_ms 1133, render/thread/ms 140, render/busy/ms 138, stage/render/prepare_assets 113, pipelines/driver_ms 97.42, main/busy/ms 91.99, stage/main/update 55.98, stage/main/post_update 19.70, stage/render/render 16.66, render/graph/ms 14.41 |
| warm-up | 3.76 | 26.9 | pipelines/compile_ms 2265, pipelines/front_ms 2163, pipelines/wait_ms 459, render/thread/ms 137, render/busy/ms 136, stage/render/prepare_assets 120, pipelines/driver_ms 102, main/busy/ms 36.09, stage/main/post_update 20.24, terrain/update/ms 10.30, pipelines/lock_ms 10.06, stage/render/render 9.97 |
| warm-up | 3.85 | 58.3 | pipelines/compile_ms 134, pipelines/front_ms 126, render/thread/ms 72.07, render/busy/ms 69.50, main/busy/ms 69.37, stage/main/update 47.84, stage/render/prepare_write_phase_buffers 23.76, stage/render/prepare_assets 18.62, stage/main/post_update 17.70, stage/render/render 13.89, encode//sum 10.93, render/graph/ms 10.70 |
| warm-up | 3.93 | 80.2 | pipelines/compile_ms 829, pipelines/front_ms 784, render/thread/ms 119, render/busy/ms 116, main/busy/ms 94.41, stage/main/update 77.34, stage/render/prepare_assets 59.68, pipelines/driver_ms 45.71, stage/render/prepare_write_phase_buffers 23.44, stage/render/render 19.11, render/graph/ms 15.99, stage/render/graph_submit 12.97 |
| warm-up | 3.99 | 66.3 | pipelines/compile_ms 1424, pipelines/front_ms 1362, render/thread/ms 96.11, render/busy/ms 93.97, stage/render/prepare_assets 72.94, pipelines/driver_ms 61.98, main/busy/ms 50.90, stage/main/update 37.51, stage/render/render 14.73, render/graph/ms 11.90, stage/render/graph_submit 9.00, encode//sum 7.46 |
| warm-up | 4.08 | 82.2 | pipelines/compile_ms 964, pipelines/front_ms 936, render/thread/ms 90.63, render/busy/ms 89.04, stage/render/prepare_assets 76.34, pipelines/driver_ms 28.19, main/busy/ms 17.77, stage/render/render 8.57, render/graph/ms 6.66, stage/main/update 6.56, encode//sum 6.02, stage/main/post_update 5.62 |
| warm-up | 4.11 | 34.4 | pipelines/compile_ms 369, pipelines/front_ms 355, render/thread/ms 83.34, render/busy/ms 81.74, stage/render/prepare_assets 62.54, main/busy/ms 17.00, stage/render/render 15.89, pipelines/driver_ms 14.58, render/graph/ms 14.21, stage/render/graph_submit 12.34, encode/submit 10.40, stage/main/update 8.07 |
| warm-up | 4.17 | 26.7 | pipelines/compile_ms 826, pipelines/front_ms 779, pipelines/driver_ms 46.89, main/busy/ms 36.55, render/thread/ms 32.33, render/busy/ms 30.64, stage/main/update 25.97, stream/drive/ms 19.91, stage/render/render 10.34, stage/render/prepare_resources 9.32, render/graph/ms 8.19, encode//sum 7.84 |
| warm-up | 4.26 | 94.2 | pipelines/compile_ms 228, pipelines/front_ms 219, main/busy/ms 168, stage/main/update 151, render/thread/ms 106, render/busy/ms 104, stage/render/prepare_resources 50.67, stream/drive/ms 27.43, stage/render/render 24.53, render/graph/ms 13.99, stage/render/prepare_bind_groups 13.75, stage/render/graph_submit 10.63 |
| warm-up | 4.45 | 192.4 | pipelines/compile_ms 3311, pipelines/front_ms 3213, pipelines/wait_ms 216, main/busy/ms 178, render/thread/ms 173, render/busy/ms 171, stage/main/update 160, pipelines/driver_ms 97.96, stage/render/prepare_assets 75.92, stage/render/prepare_resources 49.64, stream/drive/ms 33.21, stage/render/render 25.29 |
| warm-up | 4.48 | 25.7 | pipelines/compile_ms 3194, pipelines/front_ms 3089, pipelines/wait_ms 217, pipelines/driver_ms 105, render/thread/ms 104, render/busy/ms 103, stage/render/prepare_assets 77.82, main/busy/ms 67.52, stage/main/update 46.35, stream/drive/ms 34.96, stage/render/render 17.67, render/graph/ms 15.85 |
| warm-up | 4.56 | 79.0 | pipelines/compile_ms 461, pipelines/front_ms 435, pipelines/wait_ms 173, main/busy/ms 78.94, render/thread/ms 67.25, render/busy/ms 60.24, stage/main/update 49.28, stage/render/render 33.49, stream/drive/ms 28.96, render/graph/ms 25.94, pipelines/driver_ms 25.93, stage/main/post_update 17.48 |
| warm-up | 4.62 | 66.4 | pipelines/compile_ms 486, pipelines/front_ms 468, pipelines/wait_ms 277, render/thread/ms 96.35, render/busy/ms 89.29, main/busy/ms 63.10, stage/render/render 37.26, stage/main/update 36.49, stage/render/prepare_assets 33.73, render/graph/ms 29.59, stage/main/post_update 20.65, encode//sum 20.26 |
| warm-up | 4.70 | 78.5 | pipelines/compile_ms 1647, pipelines/front_ms 1600, pipelines/wait_ms 446, render/thread/ms 115, render/busy/ms 114, main/busy/ms 70.05, pipelines/driver_ms 47.13, stage/render/prepare_assets 46.48, stage/main/update 36.65, stage/render/render 34.13, encode//sum 33.46, render/graph/ms 31.91 |
| warm-up | 4.74 | 40.7 | pipelines/compile_ms 2318, pipelines/front_ms 2253, pipelines/wait_ms 357, render/thread/ms 101, render/busy/ms 98.96, main/busy/ms 69.74, pipelines/driver_ms 65.14, stage/render/prepare_assets 40.36, stage/main/post_update 36.00, stage/render/render 24.20, stage/main/update 23.14, encode//sum 22.71 |
| warm-up | 4.79 | 43.8 | pipelines/compile_ms 1238, pipelines/front_ms 1200, pipelines/wait_ms 73.07, render/thread/ms 58.57, render/busy/ms 57.01, main/busy/ms 45.86, pipelines/driver_ms 37.94, stage/main/post_update 27.93, stage/render/prepare_assets 25.51, stage/main/update 12.31, stage/render/render 11.65, encode//sum 11.48 |
| warm-up | 4.85 | 57.4 | pipelines/compile_ms 849, pipelines/front_ms 824, pipelines/wait_ms 118, render/thread/ms 73.19, render/busy/ms 71.49, main/busy/ms 59.85, stage/main/post_update 41.83, pipelines/driver_ms 25.24, stage/render/prepare_assets 23.44, stage/render/prepare_write_phase_buffers 15.30, stage/main/update 12.73, encode//sum 11.76 |
| warm-up | 4.88 | 37.2 | pipelines/compile_ms 1003, pipelines/front_ms 972, pipelines/wait_ms 147, render/thread/ms 81.68, render/busy/ms 79.82, main/busy/ms 72.67, stage/main/post_update 41.92, pipelines/driver_ms 31.04, stage/render/prepare_assets 27.56, stage/main/update 22.63, stage/render/prepare_write_phase_buffers 15.04, stage/render/render 13.43 |
| warm-up | 4.94 | 57.2 | pipelines/compile_ms 1209, pipelines/front_ms 1170, pipelines/wait_ms 85.67, render/thread/ms 67.61, main/busy/ms 67.11, render/busy/ms 65.71, stage/main/update 40.26, pipelines/driver_ms 38.52, stage/render/prepare_assets 30.12, stage/render/render 17.95, stage/main/post_update 16.76, render/graph/ms 15.12 |
| warm-up | 4.99 | 55.4 | pipelines/compile_ms 1302, pipelines/front_ms 1265, main/busy/ms 90.98, render/thread/ms 76.94, render/busy/ms 75.11, stage/main/update 63.48, stage/render/prepare_assets 44.77, pipelines/driver_ms 36.73, stage/render/render 17.96, stage/main/post_update 17.64, render/graph/ms 15.84, encode//sum 14.39 |
| warm-up | 5.04 | 47.0 | pipelines/compile_ms 1001, pipelines/front_ms 974, main/busy/ms 67.55, render/thread/ms 66.16, render/busy/ms 64.56, stage/main/update 47.74, stage/render/prepare_assets 36.73, pipelines/driver_ms 26.66, stream/drive/ms 15.48, stage/render/render 12.57, stage/main/post_update 11.79, encode//sum 11.77 |
| warm-up | 5.13 | 26.5 | pipelines/compile_ms 190, pipelines/front_ms 183, render/thread/ms 35.49, main/busy/ms 33.98, render/busy/ms 33.84, stage/main/update 21.87, encode//sum 16.02, stage/render/render 15.21, stream/drive/ms 14.97, render/graph/ms 12.93, stage/render/graph_submit 10.36, stage/render/prepare_assets 8.82 |
| warm-up | 5.21 | 40.1 | pipelines/compile_ms 857, pipelines/front_ms 832, main/busy/ms 44.33, render/thread/ms 37.44, render/busy/ms 35.80, stage/main/update 28.38, pipelines/driver_ms 25.60, encode//sum 16.13, stage/render/render 13.10, stage/main/post_update 11.08, render/graph/ms 11.06, stage/render/prepare_assets 10.24 |
| warm-up | 5.31 | 32.1 | pipelines/compile_ms 283, pipelines/front_ms 273, render/thread/ms 50.18, render/busy/ms 48.57, main/busy/ms 34.29, pipelines/wait_ms 31.31, encode//sum 17.69, stage/main/update 14.67, stage/render/render 13.97, render/graph/ms 11.99, stage/main/post_update 11.93, pipelines/driver_ms 9.85 |
| warm-up | 5.33 | 26.4 | pipelines/compile_ms 480, pipelines/front_ms 460, pipelines/wait_ms 97.35, render/thread/ms 52.96, render/busy/ms 51.41, main/busy/ms 38.86, pipelines/driver_ms 19.56, encode//sum 18.13, stage/main/update 16.75, stage/main/post_update 13.98, stage/render/render 13.85, stage/render/prepare_resources 13.09 |
| warm-up | 5.36 | 27.1 | pipelines/compile_ms 702, pipelines/front_ms 677, pipelines/wait_ms 104, render/thread/ms 48.85, render/busy/ms 47.32, main/busy/ms 40.03, pipelines/driver_ms 24.35, stage/main/update 20.68, encode//sum 16.11, stage/main/post_update 14.01, stage/render/prepare_resources 13.83, stage/render/render 13.50 |
| warm-up | 5.41 | 48.1 | pipelines/compile_ms 794, pipelines/front_ms 772, render/thread/ms 70.00, render/busy/ms 68.46, main/busy/ms 59.67, stage/main/post_update 30.54, pipelines/wait_ms 25.16, stage/main/update 23.91, pipelines/driver_ms 22.82, stage/render/prepare_resources 18.48, encode//sum 14.34, stage/render/render 13.49 |
| warm-up | 5.45 | 36.7 | pipelines/compile_ms 1134, pipelines/front_ms 1107, render/thread/ms 68.84, render/busy/ms 67.46, main/busy/ms 51.66, stage/main/post_update 28.80, pipelines/driver_ms 26.41, stage/render/prepare_assets 23.74, stage/main/update 17.30, encode//sum 15.76, stage/render/render 11.83, stage/render/prepare_resources 11.62 |
| warm-up | 5.63 | 37.2 | render/thread/ms 51.86, render/busy/ms 49.83, main/busy/ms 35.59, encode//sum 20.68, stage/render/prepare_assets 18.85, stage/render/render 18.50, stage/main/update 17.18, render/graph/ms 15.60, stage/main/post_update 12.91, stage/render/graph_submit 11.01, encode/submit 6.52, encode/finish_wall 4.53 |
| warm-up | 5.68 | 43.1 | render/thread/ms 57.76, render/busy/ms 56.27, main/busy/ms 36.57, stage/render/prepare_assets 32.53, stage/main/update 20.02, encode//sum 18.26, stage/render/render 13.13, render/graph/ms 10.86, stage/main/post_update 10.65, stage/render/graph_submit 7.65, encode/submit 3.90, encode/finish_wall 3.75 |
| warm-up | 5.71 | 35.0 | render/thread/ms 51.16, render/busy/ms 50.05, stage/render/prepare_assets 33.11, main/busy/ms 31.74, stage/main/update 18.92, encode//sum 13.70, stage/render/render 8.68, stage/main/post_update 7.84, render/graph/ms 7.12, stage/render/graph_submit 5.33, encode/finish_wall 3.08, stream/drive/ms 2.54 |
| warm-up | 5.95 | 26.9 | pipelines/compile_ms 129, pipelines/front_ms 125, render/thread/ms 36.12, render/busy/ms 35.00, main/busy/ms 32.67, stage/main/update 20.49, encode//sum 16.90, stage/render/render 11.59, stage/render/prepare_assets 11.22, render/graph/ms 10.10, stage/main/post_update 8.62, stage/render/graph_submit 7.82 |
| warm-up | 6.14 | 29.6 | render/thread/ms 30.71, main/busy/ms 29.70, render/busy/ms 29.36, stage/main/update 19.07, encode//sum 15.18, stage/render/render 12.91, render/graph/ms 11.16, stage/render/prepare_assets 9.45, stage/render/graph_submit 8.90, stage/main/post_update 7.74, encode/submit 5.07, encode/finish_wall 3.90 |
| warm-up | 6.37 | 31.9 | render/thread/ms 43.83, render/busy/ms 41.85, main/busy/ms 38.75, stage/render/render 17.61, encode//sum 17.53, stage/main/update 16.73, stage/main/post_update 16.55, render/graph/ms 15.27, stage/render/prepare_assets 13.75, stage/render/graph_submit 12.09, encode/submit 6.77, encode/finish_wall 5.45 |
| warm-up | 6.40 | 31.9 | render/thread/ms 42.32, render/busy/ms 41.09, main/busy/ms 30.19, stage/render/prepare_assets 21.83, stage/main/update 15.24, encode//sum 12.12, stage/main/post_update 9.18, stage/render/render 8.08, render/graph/ms 6.57, stage/render/prepare_meshes 5.94, stage/render/graph_submit 4.62, stream/drive/ms 3.79 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\f_scene\balmora@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (59 samples): GPU load **98%** (min 94%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.2 W · 73 °C (max 77) · VRAM 4928 MiB · throttled 100% of samples
