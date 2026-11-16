# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1478 frames over 15.0 s · **median 10.06 ms (99 fps)** · mean 10.15 · p95 11.46 · p99 12.14 (1% low 82 fps) · worst 18.56 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 8.0/min)
- warm-up: 281 frames over 8.2 s, worst 2502.1 ms
- hitches (> 25.1 ms): **0** measured, 47 in warm-up
- timed GPU passes: 8.92 ms of a 10.15 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.06 ms, limit 11 ms
- pass worst frame 18.56 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.809 | 2.828 | 2.937 | 0.99 | 0.067 |
| dlss_super_resolution | 2.042 | 2.056 | 2.074 | 0.99 | 0.185 |
| early prepass | 0.789 | 0.794 | 1.048 | 0.99 | 0.089 |
| atmosphere_luts | 0.414 | 0.416 | 0.710 | 0.99 | 0.003 |
| bloom | 0.317 | 0.319 | 0.335 | 0.99 | 0.063 |
| main_transmissive_pass_3d | 0.264 | 0.266 | 0.294 | 0.99 | 0.008 |
| water_velocity | 0.244 | 0.246 | 0.270 | 0.99 | 0.002 |
| ssao | 0.217 | 0.219 | 0.368 | 0.99 | 0.007 |
| water_sspr | 0.206 | 0.207 | 0.216 | 0.99 | 0.002 |
| per_view_shadow_pass_cascade1 | 0.174 | 0.704 | 0.985 | 0.25 | 0.049 |
| atmos_froxel_inject | 0.150 | 0.151 | 0.158 | 0.99 | 0.001 |
| clouds_trace | 0.150 | 0.151 | 0.173 | 0.99 | 0.001 |
| water_fft | 0.138 | 0.138 | 0.140 | 0.99 | 0.003 |
| uniform_allocation | 0.123 | 0.124 | 0.154 | 0.99 | 0.023 |
| upscaling | 0.114 | 0.115 | 0.121 | 0.99 | 0.004 |
| clouds_composite | 0.114 | 0.115 | 0.117 | 0.99 | 0.002 |
| bin_unpacking | 0.100 | 0.100 | 0.114 | 0.99 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.093 | 0.99 | 0.003 |
| clustering | 0.069 | 0.070 | 0.081 | 0.99 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.055 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.422 | 0.418 | 0.13 | 0.001 |
| clouds_resolve | 0.051 | 0.051 | 0.052 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.032 | 0.032 | 0.037 | 0.99 | 0.005 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.225 | 0.238 | 0.13 | 0.004 |
| main_transparent_pass_3d | 0.027 | 0.027 | 0.034 | 0.99 | 0.073 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.99 | 0.002 |
| atmosphere_environment | 0.012 | 0.012 | 0.014 | 0.99 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.008 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.006 | 0.99 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.13 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.040 | 0.040 | 0.00 | 0.007 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.016 | 0.016 | 0.00 | 0.005 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.249 | 10.565 | 17.66 | 13670.6 |
| encode//sum | 8.213 | 10.737 | 43.96 | 12138.2 |
| render/busy/ms | 7.828 | 8.833 | 16.99 | 11569.5 |
| main/busy/ms | 7.260 | 8.345 | 15.44 | 10730.1 |
| stage/render/render | 5.625 | 6.533 | 15.07 | 8314.5 |
| render/graph/ms | 4.912 | 5.793 | 14.27 | 7260.5 |
| stage/main/post_update | 4.333 | 5.392 | 12.75 | 6403.5 |
| stage/render/graph_submit | 3.912 | 4.768 | 13.36 | 5782.7 |
| encode/submit | 2.109 | 2.641 | 5.12 | 3117.4 |
| encode/finish_wall | 1.853 | 2.321 | 9.96 | 2738.5 |
| stage/main/update | 1.848 | 2.143 | 3.85 | 2732.0 |
| stage/render/prepare_views | 0.975 | 2.464 | 3.92 | 1440.5 |
| stage/render/camera_driver | 0.941 | 1.095 | 1.54 | 1390.8 |
| render/acquire/ms | 0.931 | 2.424 | 3.88 | 1375.4 |
| stage/render/prepare_write_phase_buffers | 0.596 | 0.775 | 1.07 | 881.4 |
| render/present/ms | 0.491 | 0.583 | 2.06 | 725.7 |
| stage/render/prepare_assets | 0.453 | 0.585 | 1.65 | 668.9 |
| bigspace/total/ms | 0.419 | 1.099 | 2.66 | 619.5 |
| stage/render/prepare_resources | 0.358 | 0.445 | 0.69 | 529.2 |
| stage/main/pre_update | 0.314 | 0.422 | 0.61 | 463.8 |
| stage/main/first | 0.291 | 0.357 | 1.46 | 430.5 |
| terrain/update/ms | 0.233 | 0.301 | 0.46 | 344.0 |
| stage/render/prepare_bind_groups | 0.208 | 0.309 | 0.61 | 307.2 |
| terrain/select/ms | 0.203 | 0.262 | 0.43 | 299.7 |
| stream/drive/ms | 0.193 | 0.274 | 0.54 | 285.3 |
| stage/render/prepare_meshes | 0.190 | 0.240 | 0.45 | 280.8 |
| stage/main/fixed_main_loop | 0.184 | 0.336 | 0.98 | 271.7 |
| stage/render/prepare_flush | 0.182 | 0.357 | 0.71 | 269.6 |
| bigspace/high_precision/ms | 0.176 | 0.708 | 2.43 | 260.3 |
| stage/render/extract_commands | 0.165 | 0.199 | 0.36 | 244.4 |
| stream/tick/ms | 0.165 | 0.241 | 0.46 | 243.8 |
| far/select/ms | 0.165 | 0.241 | 0.49 | 243.4 |
| stage/render/prepare_batch_phases | 0.160 | 0.216 | 0.51 | 237.2 |
| controller/feed/ms | 0.155 | 0.209 | 0.42 | 229.3 |
| controller/move/ms | 0.143 | 0.217 | 0.55 | 211.8 |
| stage/render/create_views | 0.142 | 0.183 | 0.34 | 210.3 |
| stage/main/last | 0.115 | 0.165 | 2.35 | 169.3 |
| anim/evaluate/ms | 0.106 | 0.221 | 1.95 | 157.0 |
| bigspace/low_precision/ms | 0.106 | 0.224 | 2.42 | 156.7 |
| stage/render/queue | 0.102 | 0.135 | 0.30 | 150.6 |
| stage/render/specialize | 0.091 | 0.121 | 0.43 | 134.0 |
| anim/evaluate_cpu/ms | 0.056 | 0.072 | 0.37 | 83.2 |
| encode/resolve_diagnostics/sum | 0.054 | 0.080 | 0.22 | 79.3 |
| stage/render/phase_sort | 0.053 | 0.106 | 0.20 | 77.9 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.076 | 0.36 | 69.6 |
| physics/step/ms | 0.029 | 0.056 | 0.11 | 42.8 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.040 | 0.93 | 41.2 |
| weather/map/ms | 0.025 | 0.032 | 0.25 | 36.8 |
| stage/render/cleanup | 0.024 | 0.033 | 0.32 | 35.2 |
| encode/camera_driver_span_end/sum | 0.022 | 0.036 | 0.15 | 33.0 |
| weather/eval/ms | 0.012 | 0.015 | 0.21 | 17.1 |
| script/update/ms | 0.007 | 0.009 | 0.05 | 10.2 |
| controller/avoid/ms | 0.006 | 0.008 | 0.14 | 9.0 |
| script/input/ms | 0.005 | 0.006 | 0.19 | 7.7 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 7.1 |
| anim/apply/ms | 0.004 | 0.005 | 0.03 | 6.1 |
| controller/write/ms | 0.002 | 0.002 | 0.13 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.11 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| crowd/c1/count | count | 122 | 123 | 123 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.51 | 9.92 | 2.03 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.93 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.15 | 0.05 |
| encode/finish_critical | ms | 1.51 | 9.92 | 2.03 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.22 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.12 | 16.99 | 10.19 |
| gpu/timed/ms | ms | 10.12 | 11.38 | 10.19 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 13.3 k | 13.3 k | 13.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 485.99 M | 485.99 M | 485.99 M |
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
| phases/opaque3d/batch_sets | count | 21 | 21 | 21 |
| phases/opaque3d/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 2 | 20 | 2 |
| phases/shadow/bins | count | 1887 | 2194 | 1887 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5746 | 6072 | 5746 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 17 | 20 | 17 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 290 | 290 | 290 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 0 | 39 | 0 |
| preprocess/bind_groups_created | count | 0 | 39 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 129 | 129 |
| process/cpu_usage | % | 25.48 | 37.98 | 29.81 |
| process/mem_usage | GiB | 4.62 | 4.69 | 4.69 |
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
| started_load_count |  | 3703 | 3705 | 3705 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
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
| system/cpu_usage | % | 30.14 | 44.98 | 40.21 |
| system/mem_usage | % | 48.70 | 48.96 | 48.93 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2544 | 2544 | 2544 |
| textures/images | count | 2009 | 2009 | 2009 |
| views/all/batch_sets | count | 54 | 72 | 54 |
| views/all/bins | count | 5059 | 5366 | 5059 |
| views/all/count | count | 3 | 5 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.0 k | 15.4 k | 15.0 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9498 | 15.5 k | 9498 |
| views/cascade/batch_sets | count | 2 | 20 | 2 |
| views/cascade/bins | count | 1887 | 2194 | 1887 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5746 | 6072 | 5746 |
| views/cascade/pipelines | count | 17 | 20 | 17 |
| views/cascade/work_items | count | 36 | 6072 | 36 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 52 | 52 | 52 |
| views/main/bins | count | 3172 | 3172 | 3172 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 48 | 48 | 48 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 151 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.35 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 31.50 | 34 | 46.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1478 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1478 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1478 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.61 | 1 | 900 |
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

## Hitches (> 25.1 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2502.1 | main/busy/ms 2474, stage/main/post_update 2372, stage/main/update 95.05, surface/advance_cpu_ms 67.62, stage/main/pre_update 2.94, stage/main/last 1.16, surface/cell_upload_ms 0.69, stage/main/first 0.57, stream/drive/ms 0.51, stage/main/fixed_main_loop 0.34, weather/map/ms 0.15, stream/tick/ms 0.14 |
| warm-up | 3.18 | 682.5 | main/busy/ms 2606, stage/main/post_update 2395, render/thread/ms 654, render/busy/ms 498, stage/render/prepare_assets 472, stage/main/update 198, stage/render/prepare_views 155, render/acquire/ms 154, surface/advance_cpu_ms 67.62, stage/render/prepare_resources 12.81, encode//sum 9.59, stage/render/render 8.72 |
| warm-up | 3.25 | 68.2 | render/thread/ms 704, render/busy/ms 545, stage/render/prepare_assets 499, stage/render/prepare_views 156, render/acquire/ms 155, main/busy/ms 153, stage/main/update 108, stage/main/post_update 34.16, pipelines/compile_ms 21.55, stage/render/render 20.56, pipelines/front_ms 20.49, terrain/update/ms 17.34 |
| warm-up | 3.28 | 25.7 | pipelines/compile_ms 110, pipelines/front_ms 103, render/thread/ms 72.95, render/busy/ms 63.30, main/busy/ms 43.18, stage/render/render 28.32, stage/render/prepare_assets 28.01, stage/main/post_update 26.82, terrain/update/ms 22.76, render/graph/ms 18.40, pipelines/wait_ms 16.88, render/present/ms 8.96 |
| warm-up | 3.36 | 76.7 | pipelines/wait_ms 1394, pipelines/compile_ms 740, pipelines/front_ms 696, render/thread/ms 87.45, render/busy/ms 77.60, pipelines/driver_ms 44.27, main/busy/ms 39.15, stage/main/post_update 28.37, stage/render/prepare_resources 27.21, stage/render/prepare_flush 24.43, stage/render/render 19.42, terrain/update/ms 14.67 |
| warm-up | 3.48 | 128.0 | pipelines/wait_ms 6436, pipelines/compile_ms 2159, pipelines/front_ms 2049, render/thread/ms 188, render/busy/ms 184, pipelines/driver_ms 110, main/busy/ms 96.22, stage/main/post_update 56.20, stage/render/prepare_resources 49.58, stage/render/prepare_flush 45.82, stage/render/render 36.38, render/graph/ms 33.67 |
| warm-up | 3.52 | 32.9 | pipelines/wait_ms 5688, pipelines/compile_ms 2046, pipelines/front_ms 1956, render/thread/ms 154, render/busy/ms 151, main/busy/ms 143, pipelines/driver_ms 89.93, stage/main/update 82.69, stage/main/post_update 47.90, stage/render/prepare_assets 42.04, stage/render/render 39.09, render/graph/ms 36.06 |
| warm-up | 3.64 | 125.0 | pipelines/compile_ms 1839, pipelines/front_ms 1666, pipelines/wait_ms 1025, encode//sum 212, pipelines/driver_ms 173, render/thread/ms 118, render/busy/ms 116, main/busy/ms 83.95, stage/render/prepare_assets 69.34, stage/main/update 58.45, stage/render/render 39.72, render/graph/ms 37.29 |
| warm-up | 3.70 | 36.7 | pipelines/compile_ms 695, pipelines/front_ms 655, pipelines/wait_ms 87.74, render/thread/ms 53.28, render/busy/ms 51.13, pipelines/driver_ms 39.60, stage/render/prepare_assets 26.72, main/busy/ms 19.54, encode//sum 12.53, stage/render/render 10.68, terrain/update/ms 9.74, stage/render/prepare_bind_groups 9.62 |
| warm-up | 3.79 | 32.3 | pipelines/compile_ms 356, pipelines/front_ms 335, main/busy/ms 49.79, render/thread/ms 47.89, render/busy/ms 46.16, stage/main/update 40.84, stream/drive/ms 37.70, stage/render/prepare_assets 26.47, pipelines/driver_ms 20.41, stage/render/render 14.60, render/graph/ms 12.68, encode//sum 8.02 |
| warm-up | 3.85 | 56.0 | pipelines/compile_ms 271, pipelines/front_ms 260, render/thread/ms 67.02, render/busy/ms 65.22, main/busy/ms 50.12, stage/render/prepare_assets 43.68, stage/main/update 41.18, stream/drive/ms 37.77, stage/render/render 16.06, render/graph/ms 14.00, pipelines/driver_ms 11.14, stage/render/graph_submit 9.22 |
| warm-up | 3.91 | 46.6 | pipelines/compile_ms 417, pipelines/front_ms 401, render/thread/ms 45.70, render/busy/ms 42.47, main/busy/ms 37.62, stage/main/update 26.10, stream/drive/ms 22.55, stage/render/prepare_assets 20.14, stage/render/render 18.18, pipelines/driver_ms 15.57, render/graph/ms 14.08, stage/render/graph_submit 8.30 |
| warm-up | 3.95 | 41.9 | pipelines/compile_ms 283, pipelines/front_ms 271, render/thread/ms 65.39, render/busy/ms 63.42, stage/render/prepare_assets 40.31, main/busy/ms 38.83, stage/main/post_update 17.93, pipelines/driver_ms 12.37, stage/render/render 12.17, render/graph/ms 9.38, stage/main/update 8.82, stage/main/pre_update 6.98 |
| warm-up | 4.04 | 84.4 | pipelines/compile_ms 1040, pipelines/front_ms 1006, render/thread/ms 101, render/busy/ms 98.69, main/busy/ms 93.46, stage/render/prepare_assets 65.13, stage/main/update 50.74, pipelines/driver_ms 34.04, stage/main/post_update 28.57, stream/drive/ms 21.26, stage/render/render 15.06, render/graph/ms 12.42 |
| warm-up | 4.11 | 76.7 | pipelines/compile_ms 1521, pipelines/front_ms 1473, render/thread/ms 115, render/busy/ms 111, main/busy/ms 99.76, stage/render/prepare_assets 77.45, stage/main/update 60.30, pipelines/driver_ms 47.33, stage/main/post_update 30.12, stream/drive/ms 22.20, stage/render/render 15.78, render/graph/ms 12.97 |
| warm-up | 4.18 | 68.8 | pipelines/compile_ms 1820, pipelines/front_ms 1771, main/busy/ms 137, render/thread/ms 94.56, render/busy/ms 90.34, stage/main/update 81.71, stage/render/prepare_assets 65.14, stream/drive/ms 63.68, pipelines/driver_ms 49.18, stage/main/post_update 39.37, stage/render/render 11.67, render/graph/ms 9.13 |
| warm-up | 4.37 | 187.7 | pipelines/compile_ms 3474, pipelines/front_ms 3371, pipelines/wait_ms 484, render/thread/ms 142, render/busy/ms 140, main/busy/ms 126, stage/render/prepare_assets 115, pipelines/driver_ms 103, stage/main/update 76.06, stream/drive/ms 64.43, stage/main/post_update 33.12, stage/render/render 11.36 |
| warm-up | 4.43 | 61.4 | pipelines/compile_ms 3138, pipelines/front_ms 3012, pipelines/wait_ms 650, render/thread/ms 145, render/busy/ms 143, pipelines/driver_ms 126, stage/render/prepare_assets 118, main/busy/ms 74.00, stage/main/update 31.77, stage/main/post_update 21.44, stream/drive/ms 13.93, stage/render/render 11.47 |
| warm-up | 4.48 | 50.1 | pipelines/compile_ms 1781, pipelines/front_ms 1699, pipelines/wait_ms 474, pipelines/driver_ms 81.83, render/thread/ms 79.25, render/busy/ms 77.58, main/busy/ms 63.58, stage/render/prepare_assets 54.50, stage/main/update 29.48, stage/main/post_update 15.83, stream/drive/ms 13.04, stage/main/fixed_main_loop 11.23 |
| warm-up | 4.60 | 27.9 | pipelines/compile_ms 618, pipelines/front_ms 597, main/busy/ms 66.10, pipelines/wait_ms 54.74, stage/main/update 54.46, stream/drive/ms 45.82, render/thread/ms 37.41, render/busy/ms 35.56, pipelines/driver_ms 21.06, stage/render/render 13.49, stage/render/prepare_assets 11.51, encode//sum 11.46 |
| warm-up | 4.66 | 57.0 | pipelines/compile_ms 1172, pipelines/front_ms 1140, main/busy/ms 98.54, stage/main/update 88.91, pipelines/wait_ms 84.08, stream/drive/ms 81.46, render/thread/ms 38.82, render/busy/ms 35.16, pipelines/driver_ms 32.76, stage/render/render 15.21, stage/render/prepare_assets 13.71, encode//sum 11.41 |
| warm-up | 4.71 | 49.7 | pipelines/compile_ms 1018, pipelines/front_ms 988, main/busy/ms 59.10, stage/main/update 45.52, stream/drive/ms 39.67, render/thread/ms 36.40, render/busy/ms 32.96, pipelines/wait_ms 32.48, pipelines/driver_ms 29.71, stage/render/prepare_assets 13.74, stage/render/render 12.63, encode//sum 10.73 |
| warm-up | 4.74 | 31.2 | pipelines/compile_ms 493, pipelines/front_ms 475, render/thread/ms 46.45, render/busy/ms 44.54, main/busy/ms 38.76, stage/render/render 20.17, render/graph/ms 18.01, pipelines/driver_ms 17.80, stage/main/post_update 17.61, encode//sum 11.87, stage/main/update 11.57, stage/render/prepare_assets 10.04 |
| warm-up | 4.77 | 34.1 | pipelines/compile_ms 502, pipelines/front_ms 478, main/busy/ms 92.73, stage/main/update 64.20, render/thread/ms 58.19, render/busy/ms 55.89, stage/render/render 28.53, render/graph/ms 25.82, pipelines/driver_ms 24.07, stage/main/post_update 16.20, encode//sum 14.97, stage/render/prepare_assets 14.65 |
| warm-up | 4.88 | 109.8 | pipelines/compile_ms 1535, pipelines/front_ms 1485, main/busy/ms 82.74, render/thread/ms 78.29, render/busy/ms 76.29, stage/main/update 63.13, pipelines/driver_ms 49.77, stage/render/prepare_assets 43.50, stage/render/render 18.78, render/graph/ms 16.26, encode//sum 14.61, stream/drive/ms 13.82 |
| warm-up | 4.91 | 26.6 | pipelines/compile_ms 1293, pipelines/front_ms 1258, render/thread/ms 74.23, render/busy/ms 71.50, main/busy/ms 41.60, pipelines/driver_ms 34.65, stage/render/prepare_assets 33.13, stage/main/update 18.84, stage/main/post_update 15.83, stage/render/render 14.69, render/graph/ms 12.67, encode//sum 11.26 |
| warm-up | 4.94 | 27.0 | pipelines/compile_ms 912, pipelines/front_ms 887, render/thread/ms 47.30, render/busy/ms 44.59, main/busy/ms 43.69, pipelines/driver_ms 25.01, stage/main/update 20.75, stage/main/post_update 15.05, stage/render/render 14.28, render/graph/ms 12.24, encode//sum 12.02, stage/render/graph_submit 8.78 |
| warm-up | 4.96 | 28.5 | pipelines/compile_ms 872, pipelines/front_ms 849, render/thread/ms 47.46, render/busy/ms 45.95, main/busy/ms 45.58, stage/main/update 29.69, pipelines/driver_ms 23.15, stage/render/render 21.77, render/graph/ms 19.82, stream/drive/ms 17.69, encode//sum 14.00, stage/render/camera_driver 10.34 |
| warm-up | 4.99 | 25.2 | pipelines/compile_ms 250, pipelines/front_ms 242, render/thread/ms 44.69, main/busy/ms 43.57, render/busy/ms 42.69, stage/main/update 30.64, stage/render/render 22.70, render/graph/ms 20.77, stream/drive/ms 17.96, encode//sum 13.07, stage/render/camera_driver 10.48, stage/render/graph_submit 10.09 |
| warm-up | 5.02 | 31.9 | pipelines/compile_ms 419, pipelines/front_ms 405, render/thread/ms 46.76, render/busy/ms 44.87, main/busy/ms 35.42, stage/render/prepare_assets 17.97, stage/main/update 16.10, pipelines/driver_ms 14.39, stage/main/post_update 13.50, stage/render/render 12.15, encode//sum 12.07, render/graph/ms 10.37 |
| warm-up | 5.07 | 46.3 | pipelines/compile_ms 313, pipelines/front_ms 303, render/thread/ms 69.29, render/busy/ms 60.12, main/busy/ms 54.74, stage/render/render 37.33, stage/main/update 34.61, render/graph/ms 27.77, encode//sum 26.80, stage/render/prepare_assets 18.96, stage/render/graph_submit 15.03, stage/main/post_update 12.89 |
| warm-up | 5.12 | 50.8 | pipelines/compile_ms 1004, pipelines/front_ms 976, render/thread/ms 81.06, render/busy/ms 71.54, main/busy/ms 66.52, stage/main/update 41.91, stage/render/render 38.51, encode//sum 28.40, render/graph/ms 28.33, pipelines/driver_ms 27.54, stream/drive/ms 20.09, stage/render/prepare_assets 18.42 |
| warm-up | 5.16 | 37.2 | pipelines/compile_ms 1671, pipelines/front_ms 1620, render/thread/ms 67.99, render/busy/ms 66.00, pipelines/driver_ms 51.06, main/busy/ms 50.78, stage/render/prepare_assets 23.38, stage/main/update 23.33, stage/main/post_update 21.08, stage/render/render 16.96, render/graph/ms 14.39, stream/drive/ms 13.83 |
| warm-up | 5.19 | 38.3 | pipelines/compile_ms 1311, pipelines/front_ms 1270, render/thread/ms 54.37, render/busy/ms 52.64, pipelines/driver_ms 41.17, main/busy/ms 35.11, stage/render/prepare_assets 26.76, pipelines/wait_ms 21.40, stage/main/update 15.24, stage/render/render 15.06, stage/main/post_update 13.31, render/graph/ms 12.97 |
| warm-up | 5.34 | 27.6 | pipelines/compile_ms 552, pipelines/front_ms 539, render/thread/ms 44.13, render/busy/ms 42.80, main/busy/ms 28.11, stage/render/prepare_assets 21.91, encode//sum 13.47, stage/main/update 12.40, pipelines/driver_ms 12.10, stage/render/render 10.77, stage/main/post_update 9.79, render/graph/ms 8.95 |
| warm-up | 5.38 | 27.0 | pipelines/compile_ms 329, pipelines/front_ms 321, render/thread/ms 36.64, render/busy/ms 35.23, main/busy/ms 25.52, encode//sum 15.45, stage/render/prepare_assets 13.20, stage/main/update 12.96, stage/render/render 12.48, render/graph/ms 10.60, pipelines/driver_ms 8.65, stage/render/graph_submit 7.78 |
| warm-up | 5.45 | 43.8 | pipelines/compile_ms 167, pipelines/front_ms 162, render/thread/ms 55.24, render/busy/ms 53.53, main/busy/ms 31.25, stage/render/prepare_assets 26.79, encode//sum 16.78, stage/render/render 13.19, stage/main/post_update 11.96, render/graph/ms 10.82, stage/main/update 10.22, stage/render/graph_submit 8.02 |
| warm-up | 5.48 | 30.1 | render/thread/ms 54.57, render/busy/ms 53.47, stage/render/prepare_assets 37.85, main/busy/ms 27.27, stage/main/update 12.21, encode//sum 10.97, stage/main/post_update 8.27, stage/render/render 7.98, render/graph/ms 6.49, stage/render/graph_submit 4.70, stream/drive/ms 3.86, encode/submit 2.61 |
| warm-up | 5.51 | 29.1 | render/thread/ms 46.38, render/busy/ms 45.27, stage/render/prepare_assets 28.55, main/busy/ms 27.35, stage/main/update 13.75, encode//sum 13.29, stage/render/render 8.94, stage/main/post_update 7.91, render/graph/ms 7.43, stage/render/graph_submit 5.29, stream/drive/ms 3.61, encode/finish_wall 2.92 |
| warm-up | 5.57 | 43.4 | render/thread/ms 49.50, render/busy/ms 48.12, main/busy/ms 32.22, stage/render/prepare_assets 26.68, encode//sum 17.74, stage/main/update 16.23, stage/render/render 13.13, render/graph/ms 11.03, stage/main/post_update 10.58, stage/render/graph_submit 8.32, encode/submit 5.00, stream/drive/ms 3.97 |
| warm-up | 5.62 | 30.3 | render/thread/ms 44.71, render/busy/ms 42.73, main/busy/ms 28.59, encode//sum 19.11, stage/render/prepare_assets 17.41, stage/render/render 16.65, render/graph/ms 13.96, stage/main/update 11.06, stage/main/post_update 10.81, stage/render/graph_submit 10.46, encode/submit 6.22, stream/drive/ms 5.79 |
| warm-up | 5.66 | 42.6 | render/thread/ms 58.13, render/busy/ms 56.25, main/busy/ms 34.71, stage/render/prepare_assets 30.10, encode//sum 22.11, stage/render/render 15.34, stage/main/post_update 15.02, stage/main/update 12.53, render/graph/ms 12.44, stage/render/graph_submit 8.99, encode/submit 5.06, encode/finish_wall 3.95 |
| warm-up | 5.71 | 43.0 | render/thread/ms 63.88, render/busy/ms 62.31, main/busy/ms 48.21, stage/render/prepare_assets 39.45, stage/main/update 26.03, encode//sum 18.32, stage/main/post_update 15.42, stream/drive/ms 13.46, stage/render/render 12.69, render/graph/ms 10.26, stage/render/graph_submit 7.56, encode/submit 4.16 |
| warm-up | 5.74 | 36.6 | render/thread/ms 55.18, render/busy/ms 54.07, stage/render/prepare_assets 39.08, main/busy/ms 38.40, stage/main/update 23.66, stream/drive/ms 13.36, encode//sum 12.33, stage/render/render 9.11, stage/main/post_update 8.87, pipelines/compile_ms 8.15, render/graph/ms 7.63, pipelines/front_ms 7.31 |
| warm-up | 5.89 | 27.9 | main/busy/ms 29.69, render/thread/ms 28.98, render/busy/ms 27.88, stage/main/update 18.38, encode//sum 14.67, stream/drive/ms 14.40, stage/render/prepare_assets 11.27, stage/render/render 10.34, render/graph/ms 8.82, stage/main/post_update 8.04, stage/render/graph_submit 6.26, encode/finish_wall 3.39 |
| warm-up | 6.19 | 40.6 | main/busy/ms 39.29, render/thread/ms 38.98, render/busy/ms 37.37, stage/main/update 20.76, stage/render/prepare_assets 15.06, encode//sum 13.98, stage/render/render 13.60, stage/main/post_update 13.16, render/graph/ms 11.45, stream/drive/ms 9.81, stage/render/graph_submit 8.67, encode/submit 5.36 |
| warm-up | 6.23 | 32.8 | render/thread/ms 46.51, render/busy/ms 45.42, main/busy/ms 29.99, stage/render/prepare_assets 28.70, stage/main/post_update 14.25, encode//sum 12.81, stage/main/update 10.19, stage/render/render 9.09, render/graph/ms 7.57, stage/render/graph_submit 5.32, stream/drive/ms 3.61, encode/finish_wall 2.76 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_noslab\balmora@2560x1440_off=meshslabs-2 --bench-baseline none --off meshslabs
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 0%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 95.0 W · 74 °C (max 77) · VRAM 4928 MiB · throttled 100% of samples
