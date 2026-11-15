# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: flatnormal · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1276 frames over 15.0 s · **median 11.64 ms (86 fps)** · mean 11.76 · p95 13.31 · p99 16.21 (1% low 62 fps) · worst 26.94 ms · 10 spikes over 1.5× median · 44.0 stutters/min (work 24.0/min)
- warm-up: 338 frames over 8.5 s, worst 3002.2 ms
- hitches (> 29.1 ms): **0** measured, 24 in warm-up
- timed GPU passes: 9.64 ms of a 11.76 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.64 ms, limit 11 ms
- pass worst frame 26.94 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.225 | 3.269 | 3.876 | 0.99 | 0.068 |
| dlss_super_resolution | 2.375 | 2.407 | 2.913 | 0.99 | 0.208 |
| early prepass | 0.641 | 0.649 | 0.964 | 0.99 | 0.093 |
| atmosphere_luts | 0.359 | 0.363 | 0.616 | 0.99 | 0.004 |
| bloom | 0.352 | 0.357 | 0.669 | 0.99 | 0.072 |
| main_transmissive_pass_3d | 0.291 | 0.295 | 0.635 | 0.99 | 0.009 |
| water_velocity | 0.288 | 0.292 | 0.647 | 0.99 | 0.002 |
| water_sspr | 0.261 | 0.264 | 0.581 | 0.99 | 0.002 |
| ssao | 0.204 | 0.212 | 0.358 | 0.96 | 0.009 |
| uniform_allocation | 0.176 | 0.178 | 0.334 | 0.99 | 0.032 |
| atmos_froxel_inject | 0.165 | 0.167 | 0.315 | 0.99 | 0.002 |
| clouds_trace | 0.159 | 0.162 | 0.198 | 0.99 | 0.001 |
| water_fft | 0.142 | 0.144 | 0.140 | 0.99 | 0.003 |
| upscaling | 0.125 | 0.127 | 0.147 | 0.99 | 0.005 |
| clouds_composite | 0.125 | 0.126 | 0.121 | 0.99 | 0.002 |
| bin_unpacking | 0.123 | 0.125 | 0.127 | 0.99 | 0.023 |
| tonemapping | 0.095 | 0.097 | 0.112 | 0.99 | 0.004 |
| clustering | 0.076 | 0.078 | 0.088 | 0.99 | 0.024 |
| lightprobe_irradiance_map | 0.058 | 0.472 | 0.778 | 0.12 | 0.001 |
| render_sky | 0.056 | 0.057 | 0.057 | 0.99 | 0.002 |
| clouds_resolve | 0.052 | 0.053 | 0.054 | 0.99 | 0.001 |
| water_snapshot | 0.045 | 0.045 | 0.061 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.043 | 0.041 | 0.99 | 0.007 |
| clouds_shadow | 0.032 | 0.032 | 0.037 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.031 | 0.249 | 0.519 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.033 | 0.99 | 0.001 |
| main_transparent_pass_3d | 0.027 | 0.027 | 0.029 | 0.99 | 0.078 |
| water_mips | 0.026 | 0.027 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.030 | 0.99 | 0.008 |
| atmosphere_environment | 0.012 | 0.012 | 0.014 | 0.99 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.009 | 0.99 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.004 |
| lightprobe_copy | 0.002 | 0.017 | 0.018 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.015 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.013 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.007 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 92.140 | 0.000 | 11411.76 | 117570.7 |
| render/thread/ms | 10.593 | 12.164 | 22.39 | 13517.0 |
| encode//sum | 9.562 | 12.499 | 111.44 | 12201.7 |
| render/busy/ms | 8.802 | 10.417 | 20.34 | 11232.0 |
| main/busy/ms | 8.095 | 9.684 | 23.09 | 10328.7 |
| render/graph/ms | 5.587 | 6.898 | 16.04 | 7128.8 |
| stage/main/post_update | 4.804 | 6.111 | 10.19 | 6129.7 |
| stage/render/graph_submit | 4.465 | 5.710 | 14.89 | 5697.4 |
| stage/render/render | 3.511 | 7.433 | 12.90 | 4480.7 |
| stage/render/cleanup | 2.850 | 7.379 | 16.80 | 3636.5 |
| encode/submit | 2.423 | 3.427 | 5.28 | 3091.4 |
| encode/finish_wall | 2.095 | 2.697 | 12.30 | 2673.5 |
| stage/main/update | 2.058 | 2.599 | 17.76 | 2625.9 |
| pipelines/compile_ms | 1.557 | 0.000 | 203.93 | 1986.7 |
| pipelines/front_ms | 1.500 | 0.000 | 194.65 | 1914.3 |
| stage/render/prepare_views | 1.317 | 2.877 | 4.26 | 1681.0 |
| render/acquire/ms | 1.266 | 2.817 | 4.20 | 1614.9 |
| stage/render/camera_driver | 1.053 | 1.307 | 2.21 | 1343.9 |
| stage/render/prepare_write_phase_buffers | 0.784 | 1.011 | 1.77 | 1000.7 |
| stage/render/prepare_assets | 0.535 | 0.702 | 10.26 | 682.3 |
| render/present/ms | 0.525 | 0.662 | 2.96 | 670.1 |
| stage/render/prepare_resources | 0.424 | 0.542 | 1.14 | 541.2 |
| bigspace/total/ms | 0.416 | 1.083 | 2.72 | 531.4 |
| stage/main/pre_update | 0.349 | 0.491 | 1.02 | 445.1 |
| stage/main/first | 0.326 | 0.432 | 1.19 | 415.5 |
| terrain/update/ms | 0.243 | 0.331 | 0.69 | 309.5 |
| stage/render/prepare_meshes | 0.237 | 0.334 | 0.79 | 302.5 |
| stage/main/fixed_main_loop | 0.237 | 0.444 | 1.03 | 302.4 |
| stage/render/prepare_bind_groups | 0.216 | 0.312 | 1.26 | 275.1 |
| stream/drive/ms | 0.213 | 0.340 | 0.60 | 271.9 |
| terrain/select/ms | 0.210 | 0.285 | 0.65 | 268.0 |
| controller/move/ms | 0.204 | 0.538 | 0.90 | 260.1 |
| far/select/ms | 0.185 | 0.286 | 0.62 | 236.1 |
| stage/render/prepare_batch_phases | 0.184 | 0.263 | 0.55 | 234.9 |
| stream/tick/ms | 0.182 | 0.295 | 0.55 | 231.8 |
| stage/render/create_views | 0.177 | 0.251 | 0.58 | 225.9 |
| controller/feed/ms | 0.171 | 0.232 | 0.65 | 218.4 |
| stage/render/extract_commands | 0.168 | 0.215 | 0.48 | 214.9 |
| stage/render/prepare_flush | 0.167 | 0.329 | 0.55 | 212.7 |
| bigspace/high_precision/ms | 0.160 | 0.477 | 2.43 | 204.3 |
| stage/main/last | 0.126 | 0.188 | 0.91 | 160.9 |
| stage/render/queue | 0.120 | 0.172 | 0.58 | 153.2 |
| bigspace/low_precision/ms | 0.104 | 0.237 | 2.25 | 132.2 |
| anim/evaluate/ms | 0.090 | 0.211 | 3.12 | 114.8 |
| stage/render/specialize | 0.087 | 0.123 | 0.88 | 111.3 |
| anim/evaluate_cpu/ms | 0.069 | 0.094 | 0.30 | 87.9 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.128 | 0.29 | 81.3 |
| encode/resolve_diagnostics/sum | 0.063 | 0.100 | 0.59 | 79.9 |
| pipelines/driver_ms | 0.057 | 0.000 | 9.28 | 72.3 |
| physics/step/ms | 0.040 | 0.068 | 0.72 | 51.0 |
| encode/camera_driver_span_begin/sum | 0.029 | 0.045 | 0.38 | 37.5 |
| weather/map/ms | 0.026 | 0.035 | 0.38 | 33.4 |
| physics/insert/ms | 0.025 | 0.166 | 0.80 | 32.4 |
| encode/camera_driver_span_end/sum | 0.025 | 0.042 | 0.18 | 31.4 |
| stage/render/phase_sort | 0.014 | 0.027 | 0.14 | 17.4 |
| weather/eval/ms | 0.012 | 0.017 | 0.05 | 15.7 |
| script/update/ms | 0.008 | 0.012 | 0.06 | 10.3 |
| controller/avoid/ms | 0.006 | 0.008 | 0.17 | 7.9 |
| script/ui/ms | 0.006 | 0.008 | 0.15 | 7.3 |
| script/input/ms | 0.006 | 0.007 | 0.02 | 7.1 |
| anim/apply/ms | 0.005 | 0.007 | 0.16 | 6.4 |
| controller/write/ms | 0.002 | 0.003 | 0.06 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.3 |
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
| anim/a2/count | count | 0 | 18 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 55 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 26 | 8 |
| crowd/c0/joints/count | count | 272 | 884 | 272 |
| crowd/c0/meshes/count | count | 48 | 156 | 48 |
| crowd/c1/count | count | 122 | 127 | 123 |
| crowd/c1/joints/count | count | 4848 | 5031 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1182 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 9523 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.73 | 11.46 | 1.72 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.38 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.18 | 0.01 |
| encode/finish_critical | ms | 1.73 | 11.46 | 1.72 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.59 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.48 | 23.09 | 10.86 |
| gpu/timed/ms | ms | 10.44 | 13.80 | 10.86 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1853 | 1853 | 1853 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 193.7 k | 193.7 k | 193.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 474.10 M | 474.10 M | 474.10 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 17 | 19 | 17 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 26 | 23 |
| phases/shadow/bins | count | 2028 | 2336 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5345 | 5671 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 26 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1607 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 170 | 170 | 170 |
| pipelines/waiting | count | 0 | 28 | 0 |
| preprocess/bind_groups | count | 0 | 173 | 0 |
| preprocess/bind_groups_created | count | 0 | 173 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 177 | 177 | 177 |
| process/cpu_usage | % | 32.13 | 43.75 | 34.93 |
| process/mem_usage | GiB | 4.34 | 4.45 | 4.44 |
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
| started_load_count |  | 3547 | 3549 | 3549 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 4 | 4 |
| statics/material_pipelines | count | 4 | 4 | 4 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 46.02 | 67.34 | 39.06 |
| system/mem_usage | % | 69.37 | 69.93 | 69.90 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 77 | 82 | 77 |
| views/all/bins | count | 5198 | 5505 | 5198 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 73 | 78 | 73 |
| views/all/pipelines_distinct | count | 65 | 67 | 65 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 26 | 23 |
| views/cascade/bins | count | 2028 | 2336 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5345 | 5671 | 5345 |
| views/cascade/pipelines | count | 23 | 26 | 23 |
| views/cascade/work_items | count | 5345 | 5671 | 5345 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 54 | 56 | 54 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 50 | 52 | 50 |
| views/main/work_items | count | 9462 | 9462 | 9462 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.44 | 297 | 561 |
| anim/events/count | count | 0.10 | 3 | 132 |
| assets/Image/added | count | 0.08 | 38 | 108 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.26 | 114 | 333 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.10 | 46 | 122 |
| assets/StandardMaterial/modified | count | 3.88 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 5.57 | 2538 | 7104 |
| encode//count | count | 31.23 | 33 | 39.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1276 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1276 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1276 |
| physics/inserts | count | 1.16 | 166 | 1483 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.71 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 3 | 3 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 4 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.1 ms; slowest 24, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.00 | 3002.2 | main/busy/ms 2976, stage/main/post_update 2866, stage/main/update 105, surface/advance_cpu_ms 75.84, stage/main/pre_update 2.55, stage/main/last 1.00, surface/cell_upload_ms 0.76, stream/drive/ms 0.51, stage/main/first 0.51, stage/main/fixed_main_loop 0.33, stream/tick/ms 0.15, weather/map/ms 0.09 |
| warm-up | 3.74 | 741.5 | main/busy/ms 3011, stage/main/post_update 2892, render/thread/ms 698, render/busy/ms 650, stage/render/prepare_assets 451, stage/render/prepare_resources 144, stage/main/update 108, surface/advance_cpu_ms 75.84, stage/render/prepare_views 45.07, render/acquire/ms 44.91, stage/render/render 35.76, render/graph/ms 32.37 |
| warm-up | 3.85 | 107.2 | pipelines/wait_ms 1742, render/thread/ms 706, render/busy/ms 657, stage/render/prepare_assets 452, stage/render/prepare_resources 146, pipelines/compile_ms 144, pipelines/front_ms 136, main/busy/ms 52.09, stage/render/prepare_views 45.30, render/acquire/ms 45.09, stage/render/render 39.07, stage/main/post_update 35.30 |
| warm-up | 4.11 | 89.2 | pipelines/compile_ms 243, pipelines/front_ms 228, main/busy/ms 65.83, stage/main/update 53.92, render/thread/ms 52.16, render/busy/ms 50.56, stage/render/prepare_assets 39.27, pipelines/driver_ms 15.09, stage/main/post_update 7.83, stage/render/render 6.60, render/graph/ms 4.36, stage/render/graph_submit 2.55 |
| warm-up | 4.25 | 71.3 | main/busy/ms 41.88, render/thread/ms 39.35, render/busy/ms 38.11, stage/main/update 34.38, stage/render/prepare_assets 31.17, stream/drive/ms 31.07, stage/main/post_update 4.88, stage/render/render 4.14, render/graph/ms 2.46, encode//sum 1.35, stage/render/graph_submit 1.29, stage/render/camera_driver 1.10 |
| warm-up | 4.30 | 43.6 | pipelines/wait_ms 687, pipelines/compile_ms 192, pipelines/front_ms 173, render/thread/ms 35.32, render/busy/ms 33.62, main/busy/ms 30.79, stage/render/prepare_assets 21.66, stage/main/update 19.50, pipelines/driver_ms 19.23, stream/drive/ms 16.08, stage/main/post_update 7.39, render/graph/ms 4.50 |
| warm-up | 4.33 | 30.4 | pipelines/wait_ms 2012, pipelines/compile_ms 216, pipelines/front_ms 195, render/thread/ms 48.28, render/busy/ms 46.56, stage/render/prepare_assets 33.38, main/busy/ms 22.40, pipelines/driver_ms 21.31, stage/main/update 10.11, stage/render/render 7.90, stage/main/post_update 7.55, render/graph/ms 5.56 |
| warm-up | 4.37 | 40.1 | pipelines/wait_ms 2712, pipelines/compile_ms 70.45, main/busy/ms 66.95, pipelines/front_ms 65.60, stage/main/update 47.44, render/thread/ms 46.83, render/busy/ms 45.06, stream/drive/ms 42.79, stage/render/prepare_assets 32.09, stage/main/post_update 13.52, stage/render/render 7.44, render/graph/ms 4.92 |
| warm-up | 4.47 | 95.0 | pipelines/wait_ms 3580, pipelines/compile_ms 147, pipelines/front_ms 141, render/thread/ms 67.36, main/busy/ms 67.35, render/busy/ms 65.89, stage/render/prepare_assets 54.83, stage/main/update 46.66, stream/drive/ms 42.73, stage/main/post_update 13.73, terrain/update/ms 6.43, stage/render/render 5.53 |
| warm-up | 4.52 | 55.1 | pipelines/wait_ms 4351, pipelines/compile_ms 123, pipelines/front_ms 118, render/thread/ms 86.30, render/busy/ms 60.07, stage/render/prepare_assets 51.61, main/busy/ms 44.15, stage/main/update 30.34, stage/render/cleanup 27.79, stream/drive/ms 26.58, render/present/ms 26.02, terrain/update/ms 6.38 |
| warm-up | 4.55 | 29.2 | pipelines/wait_ms 5753, pipelines/compile_ms 154, pipelines/driver_ms 82.44, pipelines/front_ms 71.73, render/thread/ms 66.81, main/busy/ms 44.78, render/busy/ms 40.56, stage/render/prepare_assets 30.90, stage/main/update 30.42, stage/render/cleanup 30.17, stream/drive/ms 26.07, render/present/ms 26.06 |
| warm-up | 4.73 | 114.3 | pipelines/wait_ms 836, render/thread/ms 80.27, main/busy/ms 55.30, render/busy/ms 48.26, stage/main/update 43.02, stream/drive/ms 38.47, stage/render/render 38.10, stage/render/prepare_assets 33.89, render/present/ms 31.71, stage/main/post_update 7.10, render/graph/ms 5.54, encode//sum 3.43 |
| warm-up | 4.86 | 63.1 | main/busy/ms 64.15, stage/main/update 56.04, stream/drive/ms 53.30, render/thread/ms 23.51, render/busy/ms 22.31, stage/render/prepare_assets 13.76, stage/main/post_update 4.60, render/graph/ms 2.64, encode//sum 2.23, stage/render/render 2.18, stage/render/cleanup 1.91, stage/render/prepare_write_phase_buffers 1.55 |
| warm-up | 4.92 | 56.1 | pipelines/wait_ms 3680, pipelines/compile_ms 137, pipelines/front_ms 132, main/busy/ms 52.70, stage/main/update 40.83, stream/drive/ms 36.68, render/thread/ms 19.41, render/busy/ms 18.33, stage/render/prepare_assets 10.18, stage/main/post_update 7.29, pipelines/driver_ms 4.70, stage/render/render 4.25 |
| warm-up | 5.18 | 30.4 | render/thread/ms 34.33, render/busy/ms 33.20, main/busy/ms 23.04, stage/render/prepare_assets 22.08, stage/main/post_update 9.48, stage/main/update 8.63, stream/drive/ms 4.37, render/graph/ms 3.64, encode//sum 2.95, stage/render/cleanup 2.78, stage/render/graph_submit 2.37, stage/render/render 2.33 |
| warm-up | 5.38 | 38.9 | render/thread/ms 54.06, render/busy/ms 52.42, stage/render/prepare_assets 32.11, main/busy/ms 24.66, stage/main/post_update 9.46, stage/main/update 8.79, render/graph/ms 5.05, stream/drive/ms 4.81, stage/render/render 3.86, stage/render/cleanup 3.45, encode//sum 3.39, stage/render/graph_submit 3.18 |
| warm-up | 5.41 | 36.1 | pipelines/wait_ms 4092, pipelines/compile_ms 166, pipelines/front_ms 158, render/thread/ms 53.82, render/busy/ms 52.36, stage/render/prepare_assets 37.74, main/busy/ms 26.25, stage/main/update 10.03, stage/main/post_update 9.84, pipelines/driver_ms 8.44, stream/drive/ms 5.53, render/graph/ms 4.55 |
| warm-up | 5.44 | 30.0 | pipelines/wait_ms 4092, pipelines/compile_ms 166, pipelines/front_ms 158, render/thread/ms 48.91, render/busy/ms 47.44, stage/render/prepare_assets 32.31, main/busy/ms 29.53, stage/main/update 12.61, stage/main/post_update 10.36, pipelines/driver_ms 8.44, stage/render/render 6.18, stream/drive/ms 4.38 |
| warm-up | 5.53 | 39.7 | pipelines/wait_ms 2851, pipelines/compile_ms 96.74, pipelines/front_ms 89.42, render/thread/ms 52.23, render/busy/ms 50.81, stage/render/prepare_assets 36.57, main/busy/ms 29.97, stage/main/update 13.32, stage/main/post_update 10.45, pipelines/driver_ms 7.32, stream/drive/ms 5.88, render/graph/ms 4.94 |
| warm-up | 5.60 | 41.9 | render/thread/ms 49.44, render/busy/ms 47.23, main/busy/ms 28.95, stage/render/prepare_assets 27.74, stage/main/post_update 13.26, stage/main/update 9.32, stage/render/render 8.66, render/graph/ms 5.80, encode//sum 4.00, stream/drive/ms 3.62, stage/render/prepare_bind_groups 3.39, stage/render/graph_submit 3.20 |
| warm-up | 5.63 | 32.5 | render/thread/ms 48.98, render/busy/ms 47.20, main/busy/ms 35.55, stage/render/prepare_assets 31.54, stage/main/update 20.12, stream/drive/ms 14.56, stage/main/post_update 9.88, stage/render/render 7.41, render/graph/ms 4.82, encode//sum 3.90, stage/render/prepare_write_phase_buffers 2.81, stage/render/graph_submit 2.37 |
| warm-up | 5.66 | 30.9 | render/thread/ms 41.00, render/busy/ms 39.69, main/busy/ms 35.54, stage/render/prepare_assets 26.64, stage/main/update 20.19, stream/drive/ms 14.32, stage/main/post_update 9.63, stage/render/render 6.96, render/graph/ms 4.86, encode//sum 3.42, stage/render/graph_submit 2.92, stage/render/prepare_write_phase_buffers 2.43 |
| warm-up | 5.79 | 29.1 | main/busy/ms 34.47, render/thread/ms 22.60, stage/main/update 22.52, render/busy/ms 21.18, stream/drive/ms 18.43, stage/render/prepare_assets 7.79, stage/render/render 7.23, stage/main/post_update 6.63, render/graph/ms 5.22, encode//sum 3.53, stage/render/graph_submit 3.17, encode/submit 2.57 |
| warm-up | 6.40 | 34.6 | pipelines/wait_ms 2483, main/busy/ms 43.28, stage/main/update 31.30, render/thread/ms 31.25, render/busy/ms 29.70, stage/render/prepare_assets 9.52, stage/main/post_update 8.30, encode//sum 7.69, render/graph/ms 7.49, pipelines/compile_ms 6.47, pipelines/front_ms 6.18, stage/render/cleanup 5.97 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\b2_off\balmora@2560x1440_off=flatnormal-1 --bench-baseline none --off flatnormal
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **98%** (min 89%) · 2580 MHz (min 2580) · memory 8001 MHz · 88.7 W · 72 °C (max 75) · VRAM 6444 MiB · throttled 100% of samples
