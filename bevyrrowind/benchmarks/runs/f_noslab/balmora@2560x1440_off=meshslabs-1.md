# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1477 frames over 15.0 s · **median 10.12 ms (99 fps)** · mean 10.16 · p95 11.51 · p99 11.95 (1% low 84 fps) · worst 16.70 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 280 frames over 8.3 s, worst 2526.7 ms
- hitches (> 25.3 ms): **0** measured, 55 in warm-up
- timed GPU passes: 8.99 ms of a 10.16 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.12 ms, limit 11 ms
- pass worst frame 16.70 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.863 | 2.875 | 2.924 | 1.00 | 0.059 |
| dlss_super_resolution | 2.048 | 2.056 | 2.077 | 1.00 | 0.184 |
| early prepass | 0.753 | 0.756 | 0.995 | 1.00 | 0.089 |
| atmosphere_luts | 0.438 | 0.440 | 0.689 | 1.00 | 0.003 |
| bloom | 0.318 | 0.319 | 0.335 | 1.00 | 0.063 |
| main_transmissive_pass_3d | 0.278 | 0.279 | 0.295 | 1.00 | 0.008 |
| water_velocity | 0.242 | 0.243 | 0.267 | 1.00 | 0.002 |
| ssao | 0.216 | 0.217 | 0.359 | 1.00 | 0.007 |
| water_sspr | 0.212 | 0.212 | 0.213 | 1.00 | 0.002 |
| per_view_shadow_pass_cascade1 | 0.178 | 0.714 | 0.967 | 0.25 | 0.046 |
| atmos_froxel_inject | 0.152 | 0.152 | 0.157 | 1.00 | 0.001 |
| clouds_trace | 0.150 | 0.150 | 0.170 | 1.00 | 0.001 |
| water_fft | 0.136 | 0.136 | 0.139 | 1.00 | 0.003 |
| uniform_allocation | 0.124 | 0.125 | 0.153 | 1.00 | 0.022 |
| clouds_composite | 0.113 | 0.114 | 0.115 | 1.00 | 0.002 |
| upscaling | 0.113 | 0.113 | 0.121 | 1.00 | 0.004 |
| bin_unpacking | 0.100 | 0.101 | 0.114 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.068 | 0.068 | 0.076 | 1.00 | 0.021 |
| lightprobe_irradiance_map | 0.053 | 0.420 | 0.419 | 0.13 | 0.001 |
| render_sky | 0.052 | 0.053 | 0.053 | 1.00 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.032 | 0.032 | 0.038 | 1.00 | 0.005 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.227 | 0.240 | 0.13 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.029 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.073 |
| water_snapshot | 0.025 | 0.026 | 0.028 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| atmosphere_environment | 0.013 | 0.013 | 0.014 | 1.00 | 0.003 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.016 | 1.00 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.004 | 1.00 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.015 | 0.13 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.13 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.043 | 0.043 | 0.00 | 0.008 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.017 | 0.017 | 0.00 | 0.010 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.251 | 10.582 | 15.79 | 13663.8 |
| encode//sum | 7.826 | 9.893 | 21.28 | 11559.6 |
| render/busy/ms | 7.558 | 8.466 | 14.50 | 11162.7 |
| main/busy/ms | 7.196 | 8.403 | 9.57 | 10629.2 |
| stage/render/render | 5.377 | 6.142 | 12.26 | 7941.2 |
| render/graph/ms | 4.687 | 5.412 | 11.58 | 6922.2 |
| stage/main/post_update | 4.358 | 5.474 | 6.80 | 6436.5 |
| stage/render/graph_submit | 3.699 | 4.432 | 10.53 | 5463.4 |
| encode/submit | 1.926 | 2.371 | 3.35 | 2844.5 |
| encode/finish_wall | 1.811 | 2.240 | 7.66 | 2674.5 |
| stage/main/update | 1.772 | 2.104 | 3.07 | 2617.6 |
| stage/render/prepare_views | 1.272 | 2.658 | 3.58 | 1878.4 |
| render/acquire/ms | 1.228 | 2.607 | 3.53 | 1813.9 |
| stage/render/camera_driver | 0.928 | 1.077 | 1.64 | 1371.4 |
| stage/render/prepare_write_phase_buffers | 0.587 | 0.779 | 1.19 | 866.9 |
| render/present/ms | 0.465 | 0.547 | 1.48 | 687.3 |
| stage/render/prepare_assets | 0.436 | 0.560 | 1.68 | 644.7 |
| bigspace/total/ms | 0.360 | 0.736 | 2.13 | 531.8 |
| stage/render/prepare_resources | 0.346 | 0.428 | 0.64 | 511.3 |
| stage/main/pre_update | 0.309 | 0.407 | 0.64 | 456.7 |
| stage/main/first | 0.288 | 0.348 | 1.18 | 425.2 |
| terrain/update/ms | 0.234 | 0.304 | 0.70 | 344.9 |
| stage/render/prepare_bind_groups | 0.214 | 0.364 | 0.62 | 315.7 |
| terrain/select/ms | 0.203 | 0.267 | 0.67 | 300.5 |
| stream/drive/ms | 0.184 | 0.252 | 0.42 | 271.7 |
| stage/render/prepare_meshes | 0.184 | 0.230 | 0.44 | 271.2 |
| stage/render/prepare_flush | 0.181 | 0.385 | 0.87 | 267.1 |
| stage/main/fixed_main_loop | 0.181 | 0.326 | 0.89 | 266.6 |
| stage/render/extract_commands | 0.162 | 0.193 | 0.47 | 239.6 |
| far/select/ms | 0.159 | 0.225 | 0.52 | 234.8 |
| stage/render/prepare_batch_phases | 0.159 | 0.208 | 0.42 | 234.7 |
| stream/tick/ms | 0.157 | 0.217 | 0.39 | 232.3 |
| controller/feed/ms | 0.154 | 0.210 | 0.57 | 228.2 |
| stage/render/create_views | 0.142 | 0.182 | 0.46 | 209.7 |
| bigspace/high_precision/ms | 0.132 | 0.313 | 1.88 | 195.0 |
| stage/main/last | 0.115 | 0.154 | 0.66 | 169.1 |
| controller/move/ms | 0.103 | 0.181 | 0.36 | 151.9 |
| stage/render/queue | 0.101 | 0.134 | 0.33 | 149.9 |
| anim/evaluate/ms | 0.101 | 0.233 | 1.74 | 148.6 |
| bigspace/low_precision/ms | 0.090 | 0.221 | 1.91 | 133.2 |
| stage/render/specialize | 0.085 | 0.111 | 0.41 | 126.0 |
| stage/render/phase_sort | 0.057 | 0.108 | 0.32 | 84.9 |
| anim/evaluate_cpu/ms | 0.057 | 0.072 | 0.39 | 84.7 |
| encode/resolve_diagnostics/sum | 0.054 | 0.080 | 0.43 | 79.6 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.081 | 0.31 | 71.1 |
| physics/step/ms | 0.028 | 0.054 | 0.09 | 41.8 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.039 | 0.23 | 39.1 |
| weather/map/ms | 0.024 | 0.032 | 0.22 | 35.8 |
| stage/render/cleanup | 0.023 | 0.032 | 0.28 | 34.2 |
| encode/camera_driver_span_end/sum | 0.022 | 0.035 | 0.09 | 32.9 |
| weather/eval/ms | 0.011 | 0.014 | 0.08 | 15.8 |
| script/update/ms | 0.006 | 0.009 | 0.11 | 9.4 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 8.5 |
| script/ui/ms | 0.005 | 0.007 | 0.19 | 7.4 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 6.9 |
| anim/apply/ms | 0.004 | 0.005 | 0.16 | 6.5 |
| controller/write/ms | 0.002 | 0.002 | 0.14 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 1.1 |
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
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.49 | 7.53 | 1.93 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.23 | 0.04 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.09 | 0.02 |
| encode/finish_critical | ms | 1.49 | 7.53 | 1.93 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.43 | 0.08 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.17 | 14.50 | 10.29 |
| gpu/timed/ms | ms | 10.17 | 11.52 | 10.29 |
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
| pipelines/total | count | 295 | 295 | 295 |
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
| process/cpu_usage | % | 24.52 | 38.94 | 27.40 |
| process/mem_usage | GiB | 4.63 | 4.70 | 4.70 |
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
| system/cpu_usage | % | 32.15 | 39.36 | 33.79 |
| system/mem_usage | % | 48.71 | 48.93 | 48.93 |
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
| anim/events/count | count | 0.10 | 2 | 141 |
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
| encode//count | count | 31.50 | 34 | 46.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1477 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1477 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1477 |
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

## Hitches (> 25.3 ms; slowest 55, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2526.7 | main/busy/ms 2500, stage/main/post_update 2393, stage/main/update 101, surface/advance_cpu_ms 70.91, stage/main/pre_update 2.60, stage/main/last 1.16, surface/cell_upload_ms 0.78, stage/main/first 0.53, stage/main/fixed_main_loop 0.39, stream/drive/ms 0.38, weather/map/ms 0.11, bigspace/total/ms 0.07 |
| warm-up | 3.28 | 758.3 | main/busy/ms 2636, stage/main/post_update 2417, render/thread/ms 730, render/busy/ms 550, stage/render/prepare_assets 514, stage/main/update 207, stage/render/prepare_views 176, render/acquire/ms 175, surface/advance_cpu_ms 70.91, stage/render/prepare_resources 21.47, stage/render/render 12.26, encode//sum 8.22 |
| warm-up | 3.38 | 92.0 | render/thread/ms 805, render/busy/ms 624, stage/render/prepare_assets 564, stage/render/prepare_views 177, render/acquire/ms 176, main/busy/ms 168, pipelines/compile_ms 142, pipelines/front_ms 131, stage/main/update 114, stage/main/post_update 38.88, stage/render/prepare_resources 26.96, stage/render/render 26.65 |
| warm-up | 3.41 | 29.8 | pipelines/compile_ms 247, pipelines/front_ms 229, render/thread/ms 98.06, render/busy/ms 89.78, main/busy/ms 51.99, stage/render/prepare_assets 51.27, pipelines/wait_ms 48.60, stage/render/render 29.50, stage/main/post_update 27.28, terrain/update/ms 24.06, render/graph/ms 21.28, pipelines/driver_ms 18.06 |
| warm-up | 3.49 | 80.2 | pipelines/wait_ms 874, pipelines/compile_ms 477, pipelines/front_ms 447, render/thread/ms 98.16, render/busy/ms 90.09, main/busy/ms 63.24, pipelines/driver_ms 30.07, stage/render/prepare_flush 28.99, stage/main/update 27.17, stage/main/post_update 23.17, stage/render/render 20.08, stage/render/prepare_resources 17.27 |
| warm-up | 3.58 | 96.1 | pipelines/wait_ms 4467, pipelines/compile_ms 1658, pipelines/front_ms 1574, render/thread/ms 168, render/busy/ms 166, main/busy/ms 159, stage/main/update 92.08, pipelines/driver_ms 84.43, stage/render/prepare_resources 60.50, stage/main/post_update 53.73, stage/render/prepare_flush 32.86, stage/render/prepare_write_phase_buffers 29.60 |
| warm-up | 3.68 | 93.7 | pipelines/wait_ms 6534, pipelines/compile_ms 2659, pipelines/front_ms 2539, render/thread/ms 161, render/busy/ms 159, main/busy/ms 131, pipelines/driver_ms 120, stage/main/update 72.35, stage/render/prepare_assets 59.02, stage/main/post_update 48.04, stage/render/prepare_resources 46.85, stage/render/render 28.82 |
| warm-up | 3.71 | 37.4 | pipelines/wait_ms 2995, pipelines/compile_ms 1715, pipelines/front_ms 1640, render/thread/ms 103, render/busy/ms 100, pipelines/driver_ms 74.54, stage/render/prepare_assets 58.22, main/busy/ms 48.27, stage/render/render 27.72, render/graph/ms 25.11, encode//sum 24.63, stage/main/post_update 17.85 |
| warm-up | 3.76 | 41.3 | pipelines/compile_ms 1055, pipelines/front_ms 1004, pipelines/wait_ms 153, render/thread/ms 73.66, render/busy/ms 71.36, pipelines/driver_ms 51.61, main/busy/ms 42.79, stage/render/prepare_assets 39.37, encode//sum 18.96, stage/render/render 18.75, stage/main/post_update 16.80, render/graph/ms 16.15 |
| warm-up | 3.85 | 33.4 | pipelines/compile_ms 396, pipelines/front_ms 368, main/busy/ms 61.34, render/thread/ms 53.80, stage/main/update 51.58, render/busy/ms 51.46, stream/drive/ms 45.69, stage/render/prepare_assets 33.09, pipelines/driver_ms 27.43, stage/render/render 13.24, render/graph/ms 10.69, encode//sum 8.42 |
| warm-up | 3.92 | 66.6 | pipelines/compile_ms 1168, pipelines/front_ms 1119, render/thread/ms 70.74, render/busy/ms 67.72, main/busy/ms 60.61, stage/render/prepare_assets 50.56, stage/main/update 49.68, pipelines/driver_ms 48.64, stream/drive/ms 45.59, pipelines/wait_ms 23.01, stage/render/render 13.82, render/graph/ms 10.64 |
| warm-up | 3.98 | 36.1 | pipelines/compile_ms 315, pipelines/front_ms 304, render/thread/ms 48.18, render/busy/ms 46.07, main/busy/ms 35.52, stage/main/update 24.19, stage/render/render 22.74, stage/render/prepare_assets 20.46, render/graph/ms 20.29, stream/drive/ms 19.53, pipelines/driver_ms 11.22, pipelines/wait_ms 9.58 |
| warm-up | 4.01 | 31.0 | pipelines/compile_ms 475, pipelines/front_ms 455, render/thread/ms 48.98, main/busy/ms 48.22, render/busy/ms 46.88, stage/render/prepare_assets 31.77, stage/main/update 24.87, stream/drive/ms 19.93, pipelines/driver_ms 19.63, stage/render/render 10.38, stage/main/post_update 10.30, stage/main/pre_update 8.72 |
| warm-up | 4.14 | 129.0 | pipelines/compile_ms 742, pipelines/front_ms 708, render/thread/ms 121, render/busy/ms 119, main/busy/ms 111, stage/main/update 76.86, stage/render/prepare_assets 75.47, pipelines/driver_ms 33.55, stage/render/render 32.27, render/graph/ms 29.16, stream/drive/ms 20.07, stage/main/post_update 14.58 |
| warm-up | 4.26 | 123.5 | pipelines/compile_ms 2036, pipelines/front_ms 1962, main/busy/ms 194, render/thread/ms 188, render/busy/ms 185, stage/main/update 137, stage/render/prepare_assets 105, encode//sum 83.96, pipelines/driver_ms 74.84, stream/drive/ms 67.31, stage/render/render 67.03, render/graph/ms 63.54 |
| warm-up | 4.44 | 176.4 | pipelines/compile_ms 4301, pipelines/front_ms 4172, pipelines/wait_ms 364, render/thread/ms 185, render/busy/ms 183, main/busy/ms 140, pipelines/driver_ms 129, stage/render/prepare_assets 128, stage/main/update 89.80, encode//sum 82.39, stream/drive/ms 67.80, stage/render/render 46.16 |
| warm-up | 4.49 | 51.1 | pipelines/compile_ms 2833, pipelines/front_ms 2750, pipelines/wait_ms 417, render/thread/ms 135, render/busy/ms 133, stage/render/prepare_assets 103, pipelines/driver_ms 82.78, main/busy/ms 47.90, stage/main/update 24.65, stage/render/render 16.68, render/graph/ms 14.26, stream/drive/ms 13.73 |
| warm-up | 4.53 | 39.3 | pipelines/compile_ms 335, pipelines/front_ms 311, render/thread/ms 74.68, render/busy/ms 72.56, pipelines/wait_ms 69.50, main/busy/ms 64.97, stage/main/update 39.35, pipelines/driver_ms 23.98, stage/render/prepare_assets 22.04, stage/render/render 17.73, render/graph/ms 15.46, stream/drive/ms 12.74 |
| warm-up | 4.57 | 48.3 | render/thread/ms 74.26, render/busy/ms 72.48, main/busy/ms 66.53, pipelines/compile_ms 61.43, pipelines/front_ms 50.29, stage/main/update 39.10, stage/render/prepare_assets 18.05, encode//sum 11.69, stage/render/prepare_resources 11.20, pipelines/driver_ms 11.14, stage/main/post_update 11.04, stage/render/render 10.83 |
| warm-up | 4.61 | 35.4 | render/thread/ms 66.38, render/busy/ms 64.96, main/busy/ms 50.76, stage/main/update 26.01, stage/render/prepare_assets 21.07, stage/main/post_update 14.09, stage/render/prepare_resources 10.25, stage/render/prepare_write_phase_buffers 10.24, stage/render/render 9.88, encode//sum 9.85, stream/drive/ms 9.48, render/graph/ms 8.22 |
| warm-up | 4.67 | 55.3 | pipelines/compile_ms 2306, pipelines/front_ms 2204, pipelines/wait_ms 320, pipelines/driver_ms 101, render/thread/ms 73.82, render/busy/ms 72.31, main/busy/ms 46.77, stage/render/prepare_assets 40.45, stage/main/update 26.07, stage/main/post_update 15.91, stage/render/render 11.03, encode//sum 9.98 |
| warm-up | 4.69 | 28.1 | pipelines/compile_ms 2549, pipelines/front_ms 2439, pipelines/wait_ms 343, pipelines/driver_ms 110, render/thread/ms 67.09, render/busy/ms 65.27, stage/render/prepare_assets 42.88, main/busy/ms 40.87, stage/main/update 23.10, stage/main/post_update 12.29, stage/render/render 11.20, encode//sum 11.00 |
| warm-up | 4.80 | 81.7 | pipelines/compile_ms 772, pipelines/front_ms 749, pipelines/wait_ms 190, main/busy/ms 91.07, stage/main/update 76.29, stream/drive/ms 70.84, render/thread/ms 34.15, render/busy/ms 32.09, pipelines/driver_ms 22.33, stage/render/prepare_assets 14.20, stage/render/render 10.38, stage/main/post_update 9.92 |
| warm-up | 4.83 | 29.7 | pipelines/compile_ms 755, pipelines/front_ms 727, pipelines/wait_ms 153, render/thread/ms 42.28, render/busy/ms 40.19, main/busy/ms 30.50, pipelines/driver_ms 27.49, stage/render/render 13.02, stage/main/post_update 12.77, stage/render/prepare_assets 11.81, render/graph/ms 10.45, stage/main/update 10.27 |
| warm-up | 4.87 | 43.3 | pipelines/compile_ms 654, pipelines/front_ms 629, render/thread/ms 63.79, render/busy/ms 61.83, main/busy/ms 55.20, pipelines/driver_ms 24.71, stage/main/update 22.56, stage/main/post_update 21.52, stage/render/render 18.24, stage/render/prepare_resources 17.84, render/graph/ms 15.87, encode//sum 12.07 |
| warm-up | 4.92 | 50.9 | pipelines/compile_ms 252, pipelines/front_ms 241, main/busy/ms 82.15, render/thread/ms 78.32, render/busy/ms 76.29, stage/main/update 50.62, stage/render/prepare_assets 22.32, stage/main/post_update 21.82, stage/render/render 17.24, stage/render/prepare_resources 16.24, render/graph/ms 14.83, encode//sum 12.43 |
| warm-up | 4.97 | 45.1 | render/thread/ms 69.32, render/busy/ms 67.59, main/busy/ms 58.61, pipelines/compile_ms 52.39, pipelines/front_ms 49.55, stage/main/update 39.34, stage/render/prepare_assets 34.71, encode//sum 13.26, stream/drive/ms 13.12, stage/main/post_update 13.06, stage/render/render 11.96, render/graph/ms 9.92 |
| warm-up | 5.00 | 27.6 | pipelines/compile_ms 652, pipelines/front_ms 633, render/thread/ms 56.03, render/busy/ms 54.43, main/busy/ms 35.73, pipelines/driver_ms 19.20, stage/render/prepare_assets 17.37, stage/main/update 15.92, encode//sum 14.88, stage/main/post_update 14.37, stage/render/render 11.61, render/graph/ms 9.70 |
| warm-up | 5.02 | 26.5 | pipelines/compile_ms 1219, pipelines/front_ms 1185, render/thread/ms 48.33, render/busy/ms 46.68, main/busy/ms 37.68, pipelines/driver_ms 33.31, stage/main/update 19.36, encode//sum 13.08, stage/render/render 12.90, stage/main/post_update 12.57, render/graph/ms 10.86, stage/render/prepare_resources 9.81 |
| warm-up | 5.05 | 26.1 | pipelines/compile_ms 992, pipelines/front_ms 964, render/thread/ms 43.51, main/busy/ms 41.73, render/busy/ms 41.70, stage/main/update 28.71, pipelines/driver_ms 28.16, stage/render/render 18.21, stream/drive/ms 16.89, render/graph/ms 16.01, encode//sum 13.49, stage/render/graph_submit 13.38 |
| warm-up | 5.08 | 30.0 | pipelines/compile_ms 782, pipelines/front_ms 755, main/busy/ms 46.27, render/thread/ms 45.69, render/busy/ms 43.93, stage/main/update 28.39, pipelines/driver_ms 26.35, stage/render/render 19.67, stream/drive/ms 17.63, render/graph/ms 17.53, encode//sum 14.43, stage/render/graph_submit 14.27 |
| warm-up | 5.11 | 28.2 | pipelines/compile_ms 591, pipelines/front_ms 572, render/thread/ms 47.99, render/busy/ms 45.82, main/busy/ms 39.72, pipelines/driver_ms 19.51, stage/main/post_update 17.99, stage/main/update 15.75, stage/render/render 14.07, encode//sum 13.80, render/graph/ms 12.07, stage/render/prepare_assets 9.20 |
| warm-up | 5.14 | 35.7 | pipelines/compile_ms 415, pipelines/front_ms 403, render/thread/ms 50.16, render/busy/ms 47.97, main/busy/ms 39.31, stage/main/update 18.10, stage/render/prepare_assets 17.32, stage/main/post_update 14.29, encode//sum 13.54, pipelines/driver_ms 11.71, stage/render/render 11.59, render/graph/ms 9.58 |
| warm-up | 5.18 | 40.4 | pipelines/compile_ms 455, pipelines/front_ms 441, render/thread/ms 59.58, render/busy/ms 57.84, main/busy/ms 40.51, stage/render/prepare_assets 29.41, stage/main/update 20.27, encode//sum 13.55, stage/main/post_update 13.54, pipelines/driver_ms 13.39, stage/render/render 11.15, render/graph/ms 8.97 |
| warm-up | 5.22 | 35.6 | pipelines/compile_ms 238, pipelines/front_ms 231, render/thread/ms 64.23, render/busy/ms 62.50, main/busy/ms 44.87, stage/render/prepare_assets 36.57, stage/main/post_update 21.78, stage/main/update 16.92, encode//sum 13.22, stage/render/render 11.13, render/graph/ms 8.95, pipelines/driver_ms 7.15 |
| warm-up | 5.26 | 39.0 | pipelines/compile_ms 1361, pipelines/front_ms 1324, render/thread/ms 58.74, render/busy/ms 57.17, main/busy/ms 46.16, pipelines/driver_ms 37.20, stage/render/prepare_assets 36.85, pipelines/wait_ms 29.75, stage/main/post_update 17.66, stage/main/update 16.45, encode//sum 13.00, stage/render/render 10.37 |
| warm-up | 5.28 | 27.3 | pipelines/compile_ms 1578, pipelines/front_ms 1535, render/thread/ms 51.31, render/busy/ms 49.69, pipelines/driver_ms 43.04, main/busy/ms 33.97, pipelines/wait_ms 30.37, stage/render/prepare_assets 24.87, stage/render/render 15.03, encode//sum 14.03, stage/main/update 13.25, render/graph/ms 12.96 |
| warm-up | 5.35 | 25.7 | pipelines/compile_ms 258, pipelines/front_ms 243, render/thread/ms 44.77, render/busy/ms 43.13, main/busy/ms 32.37, stage/main/update 16.52, pipelines/driver_ms 14.57, encode//sum 12.83, stage/render/render 12.63, stage/render/prepare_resources 11.41, stage/main/post_update 11.31, stream/drive/ms 10.67 |
| warm-up | 5.38 | 27.4 | pipelines/compile_ms 428, pipelines/front_ms 414, render/thread/ms 49.42, render/busy/ms 47.90, main/busy/ms 36.01, stage/main/update 17.93, stage/render/prepare_assets 17.01, pipelines/driver_ms 13.68, encode//sum 13.42, stage/main/post_update 12.87, stream/drive/ms 11.42, stage/render/render 11.28 |
| warm-up | 5.42 | 28.4 | pipelines/compile_ms 557, pipelines/front_ms 542, render/thread/ms 41.78, render/busy/ms 40.32, pipelines/wait_ms 35.00, main/busy/ms 27.19, stage/render/prepare_assets 18.56, pipelines/driver_ms 14.94, encode//sum 13.77, stage/render/render 11.43, stage/main/update 11.01, stage/main/post_update 10.96 |
| warm-up | 5.47 | 28.4 | pipelines/compile_ms 737, pipelines/front_ms 714, render/thread/ms 43.05, render/busy/ms 41.50, main/busy/ms 36.79, pipelines/driver_ms 23.19, encode//sum 16.71, stage/render/prepare_assets 15.94, stage/main/update 15.84, stage/main/post_update 14.35, stage/render/render 13.14, render/graph/ms 10.94 |
| warm-up | 5.51 | 32.5 | pipelines/compile_ms 1130, pipelines/front_ms 1100, render/thread/ms 54.44, render/busy/ms 53.03, main/busy/ms 37.75, pipelines/driver_ms 29.77, stage/render/prepare_assets 29.52, stage/main/update 18.43, encode//sum 15.80, stage/main/post_update 12.37, stage/render/render 11.94, render/graph/ms 9.82 |
| warm-up | 5.53 | 28.4 | pipelines/compile_ms 392, pipelines/front_ms 386, render/thread/ms 54.60, render/busy/ms 53.41, stage/render/prepare_assets 32.54, main/busy/ms 29.61, stage/main/update 12.37, stage/main/post_update 11.65, encode//sum 11.04, stage/render/render 8.85, render/graph/ms 7.25, pipelines/driver_ms 6.58 |
| warm-up | 5.59 | 35.7 | render/thread/ms 47.91, render/busy/ms 46.07, main/busy/ms 33.05, stage/render/prepare_assets 20.89, encode//sum 17.24, stage/main/update 13.76, stage/render/render 13.62, stage/main/post_update 11.28, render/graph/ms 11.22, stage/render/graph_submit 8.41, encode/submit 4.88, stream/drive/ms 4.57 |
| warm-up | 5.62 | 34.9 | pipelines/compile_ms 288, pipelines/front_ms 279, render/thread/ms 53.49, render/busy/ms 52.14, stage/render/prepare_assets 33.48, main/busy/ms 30.90, stage/main/update 13.88, encode//sum 13.29, pipelines/driver_ms 9.36, stage/render/render 9.21, stage/main/post_update 8.89, render/graph/ms 7.32 |
| warm-up | 5.65 | 29.5 | pipelines/compile_ms 288, pipelines/front_ms 279, render/thread/ms 49.88, render/busy/ms 48.76, stage/render/prepare_assets 32.69, main/busy/ms 26.08, stage/main/update 12.41, encode//sum 10.98, pipelines/driver_ms 9.36, stage/main/post_update 8.90, stage/render/render 8.05, render/graph/ms 6.64 |
| warm-up | 5.70 | 31.6 | render/thread/ms 45.11, render/busy/ms 44.06, main/busy/ms 33.52, stage/render/prepare_assets 26.17, stage/main/update 15.69, encode//sum 12.81, stage/main/post_update 12.61, stage/render/render 8.97, render/graph/ms 7.55, stage/render/graph_submit 5.73, stream/drive/ms 3.54, encode/submit 3.07 |
| warm-up | 5.73 | 30.2 | render/thread/ms 52.80, render/busy/ms 51.16, main/busy/ms 38.71, stage/render/prepare_assets 27.53, stage/main/update 18.95, encode//sum 17.13, stage/main/post_update 13.75, stage/render/render 13.00, render/graph/ms 10.68, stage/render/graph_submit 7.95, stream/drive/ms 6.00, encode/submit 4.61 |
| warm-up | 5.77 | 31.9 | render/thread/ms 47.36, render/busy/ms 45.07, main/busy/ms 34.62, encode//sum 20.43, stage/render/prepare_assets 17.14, stage/render/render 16.91, stage/main/post_update 14.52, render/graph/ms 13.87, stage/main/update 13.61, stage/render/graph_submit 9.78, stream/drive/ms 6.11, encode/submit 5.42 |
| warm-up | 5.80 | 34.0 | render/thread/ms 43.47, render/busy/ms 41.74, main/busy/ms 29.93, stage/render/prepare_assets 21.45, encode//sum 16.55, stage/main/post_update 12.74, stage/render/render 12.33, stage/main/update 10.97, render/graph/ms 10.06, stage/render/graph_submit 6.98, encode/finish_wall 3.93, stream/drive/ms 3.68 |
| warm-up | 5.83 | 27.2 | main/busy/ms 44.12, render/thread/ms 41.69, render/busy/ms 40.20, stage/main/update 26.58, stage/render/prepare_assets 21.48, encode//sum 18.51, stream/drive/ms 13.84, stage/render/render 12.67, stage/main/post_update 12.04, render/graph/ms 10.77, stage/render/graph_submit 8.79, pipelines/compile_ms 8.13 |
| warm-up | 5.86 | 28.5 | main/busy/ms 42.74, render/thread/ms 34.76, render/busy/ms 33.23, stage/main/update 25.78, encode//sum 18.18, stream/drive/ms 13.91, stage/render/prepare_assets 13.37, stage/render/render 12.91, stage/main/post_update 12.29, render/graph/ms 10.97, stage/render/graph_submit 8.86, pipelines/compile_ms 8.13 |
| warm-up | 6.00 | 28.9 | main/busy/ms 35.13, render/thread/ms 26.56, render/busy/ms 25.27, stage/main/update 22.90, stream/drive/ms 15.81, encode//sum 15.73, stage/render/render 10.36, stage/render/prepare_assets 9.26, render/graph/ms 8.89, stage/main/post_update 8.52, stage/render/graph_submit 6.88, encode/submit 3.63 |
| warm-up | 6.25 | 27.2 | render/thread/ms 31.28, main/busy/ms 30.42, render/busy/ms 29.73, encode//sum 23.88, stage/render/render 16.70, render/graph/ms 14.59, stage/main/update 13.97, stage/main/post_update 12.30, stage/render/graph_submit 11.15, stage/render/prepare_assets 7.13, encode/finish_wall 5.75, encode/submit 5.43 |
| warm-up | 6.32 | 47.2 | render/thread/ms 48.46, render/busy/ms 46.71, main/busy/ms 42.92, stage/render/prepare_assets 25.62, stage/main/update 22.64, encode//sum 16.72, stage/main/post_update 14.79, stage/render/render 14.67, render/graph/ms 12.41, stream/drive/ms 10.58, stage/render/graph_submit 9.53, encode/submit 5.67 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_noslab\balmora@2560x1440_off=meshslabs-1 --bench-baseline none --off meshslabs
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 95.7 W · 75 °C (max 77) · VRAM 4938 MiB · throttled 100% of samples
