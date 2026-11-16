# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1398 frames over 15.0 s · **median 10.67 ms (94 fps)** · mean 10.73 · p95 11.79 · p99 12.52 (1% low 80 fps) · worst 17.26 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 281 frames over 8.6 s, worst 2535.6 ms
- hitches (> 26.7 ms): **0** measured, 48 in warm-up
- timed GPU passes: 9.51 ms of a 10.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.67 ms, limit 11 ms
- pass worst frame 17.26 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.877 | 2.887 | 3.119 | 1.00 | 0.061 |
| dlss_super_resolution | 2.050 | 2.057 | 2.075 | 1.00 | 0.185 |
| early prepass | 0.801 | 0.804 | 1.039 | 1.00 | 0.087 |
| per_view_shadow_pass_cascade1 | 0.701 | 0.703 | 0.972 | 1.00 | 0.048 |
| atmosphere_luts | 0.334 | 0.336 | 0.409 | 1.00 | 0.003 |
| bloom | 0.321 | 0.322 | 0.335 | 1.00 | 0.066 |
| main_transmissive_pass_3d | 0.280 | 0.281 | 0.296 | 1.00 | 0.008 |
| water_velocity | 0.242 | 0.243 | 0.266 | 1.00 | 0.001 |
| water_sspr | 0.213 | 0.214 | 0.220 | 1.00 | 0.002 |
| ssao | 0.197 | 0.198 | 0.205 | 1.00 | 0.007 |
| atmos_froxel_inject | 0.153 | 0.153 | 0.158 | 1.00 | 0.001 |
| clouds_trace | 0.150 | 0.151 | 0.170 | 1.00 | 0.001 |
| uniform_allocation | 0.150 | 0.150 | 0.153 | 1.00 | 0.027 |
| water_fft | 0.135 | 0.135 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.115 | 0.115 | 0.122 | 1.00 | 0.004 |
| clouds_composite | 0.114 | 0.114 | 0.114 | 1.00 | 0.002 |
| bin_unpacking | 0.111 | 0.111 | 0.113 | 1.00 | 0.020 |
| tonemapping | 0.086 | 0.087 | 0.090 | 1.00 | 0.003 |
| clustering | 0.070 | 0.070 | 0.076 | 1.00 | 0.022 |
| render_sky | 0.053 | 0.053 | 0.053 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.415 | 0.418 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.039 | 0.039 | 0.038 | 1.00 | 0.006 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.227 | 0.237 | 0.13 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.028 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.074 |
| water_snapshot | 0.025 | 0.025 | 0.027 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.016 | 0.024 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 1.00 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.004 | 1.00 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.13 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.040 | 0.040 | 0.00 | 0.009 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.018 | 0.018 | 0.00 | 0.007 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.004 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.799 | 10.863 | 16.34 | 13699.5 |
| encode//sum | 8.665 | 10.797 | 24.74 | 12113.8 |
| render/busy/ms | 7.643 | 8.571 | 14.75 | 10684.4 |
| main/busy/ms | 7.312 | 8.560 | 10.11 | 10222.6 |
| stage/render/render | 5.345 | 6.141 | 12.81 | 7471.6 |
| render/graph/ms | 4.646 | 5.428 | 12.13 | 6495.2 |
| stage/main/post_update | 4.547 | 5.662 | 6.72 | 6356.8 |
| stage/render/graph_submit | 3.629 | 4.389 | 11.17 | 5073.4 |
| encode/submit | 1.931 | 2.417 | 3.37 | 2700.0 |
| encode/finish_wall | 1.736 | 2.166 | 8.33 | 2426.4 |
| stage/render/prepare_views | 1.731 | 2.657 | 4.12 | 2419.5 |
| render/acquire/ms | 1.686 | 2.607 | 4.06 | 2356.8 |
| stage/main/update | 1.680 | 2.057 | 3.38 | 2348.6 |
| stage/render/camera_driver | 0.955 | 1.120 | 1.53 | 1335.1 |
| stage/render/prepare_write_phase_buffers | 0.690 | 0.838 | 1.25 | 964.1 |
| render/present/ms | 0.471 | 0.559 | 1.39 | 658.3 |
| stage/render/prepare_assets | 0.434 | 0.557 | 1.70 | 607.1 |
| stage/render/prepare_resources | 0.394 | 0.471 | 0.66 | 551.1 |
| bigspace/total/ms | 0.367 | 0.655 | 2.25 | 512.5 |
| stage/main/pre_update | 0.311 | 0.402 | 0.64 | 434.4 |
| stage/main/first | 0.291 | 0.352 | 1.19 | 407.5 |
| terrain/update/ms | 0.232 | 0.301 | 0.69 | 324.6 |
| terrain/select/ms | 0.202 | 0.258 | 0.66 | 282.2 |
| stage/render/prepare_flush | 0.199 | 0.390 | 0.76 | 277.8 |
| stage/render/prepare_bind_groups | 0.197 | 0.291 | 0.65 | 276.1 |
| stage/main/fixed_main_loop | 0.188 | 0.317 | 0.90 | 262.6 |
| stage/render/prepare_meshes | 0.183 | 0.233 | 0.53 | 256.3 |
| stage/render/extract_commands | 0.174 | 0.209 | 0.43 | 243.0 |
| stream/drive/ms | 0.173 | 0.204 | 0.46 | 241.2 |
| controller/feed/ms | 0.165 | 0.229 | 0.45 | 231.1 |
| stage/render/prepare_batch_phases | 0.162 | 0.218 | 0.41 | 225.9 |
| far/select/ms | 0.150 | 0.191 | 0.49 | 210.3 |
| stage/render/create_views | 0.147 | 0.192 | 0.34 | 204.9 |
| stream/tick/ms | 0.147 | 0.174 | 0.43 | 204.8 |
| bigspace/high_precision/ms | 0.130 | 0.293 | 2.00 | 181.5 |
| stage/main/last | 0.119 | 0.171 | 0.78 | 166.8 |
| anim/evaluate/ms | 0.110 | 0.251 | 1.29 | 154.0 |
| controller/move/ms | 0.106 | 0.171 | 0.39 | 148.1 |
| bigspace/low_precision/ms | 0.089 | 0.219 | 0.92 | 124.6 |
| stage/render/specialize | 0.073 | 0.097 | 0.39 | 101.5 |
| stage/render/queue | 0.064 | 0.088 | 0.36 | 88.9 |
| anim/evaluate_cpu/ms | 0.061 | 0.085 | 0.38 | 86.0 |
| stage/render/phase_sort | 0.059 | 0.080 | 0.21 | 82.3 |
| encode/resolve_diagnostics/sum | 0.055 | 0.084 | 0.16 | 76.8 |
| stage/render/prepare_collect_phase_buffers | 0.051 | 0.086 | 0.32 | 70.7 |
| physics/step/ms | 0.029 | 0.053 | 0.08 | 41.0 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.040 | 0.11 | 37.0 |
| weather/map/ms | 0.025 | 0.032 | 0.23 | 34.3 |
| stage/render/cleanup | 0.024 | 0.035 | 0.33 | 34.2 |
| encode/camera_driver_span_end/sum | 0.023 | 0.037 | 0.39 | 32.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 14.9 |
| script/update/ms | 0.006 | 0.008 | 0.19 | 8.6 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 7.8 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 6.5 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.2 |
| script/ui/ms | 0.004 | 0.007 | 0.02 | 5.8 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
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
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4212 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.43 | 8.14 | 1.39 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.11 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.39 | 0.03 |
| encode/finish_critical | ms | 1.43 | 8.14 | 1.39 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.16 | 0.05 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.17 | 14.75 | 10.25 |
| gpu/timed/ms | ms | 10.17 | 11.89 | 10.25 |
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
| phases/shadow/batch_sets | count | 17 | 20 | 17 |
| phases/shadow/bins | count | 1798 | 2105 | 1798 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5089 | 5415 | 5089 |
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
| preprocess/bind_groups | count | 0 | 8 | 0 |
| preprocess/bind_groups_created | count | 0 | 8 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 160 | 160 | 160 |
| process/cpu_usage | % | 24.52 | 37.10 | 20.57 |
| process/mem_usage | GiB | 4.58 | 4.66 | 4.66 |
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
| system/cpu_usage | % | 31.14 | 43.55 | 34.08 |
| system/mem_usage | % | 56.31 | 56.54 | 56.54 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2544 | 2544 | 2544 |
| textures/images | count | 2009 | 2009 | 2009 |
| views/all/batch_sets | count | 69 | 72 | 69 |
| views/all/bins | count | 4970 | 5277 | 4970 |
| views/all/count | count | 3 | 5 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.4 k | 14.7 k | 14.4 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 14.6 k | 14.9 k | 14.6 k |
| views/cascade/batch_sets | count | 17 | 20 | 17 |
| views/cascade/bins | count | 1798 | 2105 | 1798 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5089 | 5415 | 5089 |
| views/cascade/pipelines | count | 17 | 20 | 17 |
| views/cascade/work_items | count | 5089 | 5415 | 5089 |
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
| anim/events/count | count | 0.10 | 3 | 136 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.54 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 32.25 | 34 | 45.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1398 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1398 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1398 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 900 |
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

## Hitches (> 26.7 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2535.6 | main/busy/ms 2508, stage/main/post_update 2405, stage/main/update 96.18, surface/advance_cpu_ms 65.10, stage/main/pre_update 2.53, stage/main/last 0.96, surface/cell_upload_ms 0.68, stage/main/first 0.51, stage/main/fixed_main_loop 0.42, stream/drive/ms 0.28, weather/map/ms 0.12, bigspace/total/ms 0.09 |
| warm-up | 3.29 | 751.3 | main/busy/ms 2645, stage/main/post_update 2431, render/thread/ms 723, render/busy/ms 558, stage/render/prepare_assets 522, stage/main/update 202, stage/render/prepare_views 164, render/acquire/ms 163, surface/advance_cpu_ms 65.10, stage/render/prepare_resources 21.49, encode//sum 10.00, stage/render/render 9.68 |
| warm-up | 3.38 | 89.8 | render/thread/ms 794, render/busy/ms 628, stage/render/prepare_assets 572, stage/render/prepare_views 165, render/acquire/ms 163, main/busy/ms 162, stage/main/update 111, stage/main/post_update 38.16, stage/render/prepare_resources 24.76, stage/render/render 23.95, pipelines/compile_ms 22.27, pipelines/front_ms 20.14 |
| warm-up | 3.41 | 32.8 | pipelines/compile_ms 122, pipelines/front_ms 109, render/thread/ms 103, render/busy/ms 90.07, main/busy/ms 54.01, stage/render/prepare_assets 50.40, pipelines/wait_ms 49.37, stage/render/render 38.56, stage/main/post_update 35.66, render/graph/ms 25.61, terrain/update/ms 24.05, pipelines/driver_ms 13.18 |
| warm-up | 3.47 | 63.8 | pipelines/wait_ms 1344, pipelines/compile_ms 718, pipelines/front_ms 624, pipelines/driver_ms 94.36, render/thread/ms 92.51, render/busy/ms 80.30, main/busy/ms 70.53, stage/main/post_update 33.10, stage/render/render 28.62, stage/render/prepare_flush 27.60, stage/main/pre_update 27.14, stage/render/prepare_assets 20.31 |
| warm-up | 3.62 | 150.1 | pipelines/wait_ms 5043, pipelines/compile_ms 1975, pipelines/front_ms 1845, render/thread/ms 209, render/busy/ms 207, pipelines/driver_ms 129, main/busy/ms 76.74, encode//sum 44.76, stage/render/prepare_resources 42.02, stage/render/render 41.09, stage/render/prepare_flush 39.44, render/graph/ms 38.49 |
| warm-up | 3.66 | 34.9 | pipelines/wait_ms 4199, pipelines/compile_ms 2214, pipelines/front_ms 2125, render/thread/ms 181, render/busy/ms 177, main/busy/ms 96.48, pipelines/driver_ms 88.43, stage/main/update 71.98, encode//sum 46.38, stage/render/prepare_resources 44.30, stage/render/render 41.75, render/graph/ms 39.04 |
| warm-up | 3.81 | 147.4 | pipelines/compile_ms 2444, pipelines/front_ms 2340, pipelines/wait_ms 969, render/thread/ms 145, render/busy/ms 140, pipelines/driver_ms 104, stage/render/prepare_assets 83.09, main/busy/ms 79.14, stage/main/update 55.48, stage/render/prepare_resources 39.85, stage/main/post_update 16.29, stage/render/render 13.09 |
| warm-up | 3.93 | 39.8 | main/busy/ms 63.97, render/thread/ms 59.38, render/busy/ms 57.44, pipelines/compile_ms 54.81, pipelines/front_ms 52.45, stage/main/update 47.70, stream/drive/ms 39.19, stage/render/prepare_assets 37.54, stage/render/render 13.19, render/graph/ms 10.98, encode//sum 9.23, stage/render/graph_submit 8.03 |
| warm-up | 3.97 | 42.9 | pipelines/compile_ms 358, pipelines/front_ms 341, render/thread/ms 67.82, render/busy/ms 65.87, main/busy/ms 52.85, stage/render/prepare_assets 49.58, stage/main/update 43.47, stream/drive/ms 39.35, pipelines/driver_ms 16.08, stage/render/render 11.92, render/graph/ms 9.54, encode//sum 7.98 |
| warm-up | 4.03 | 55.3 | pipelines/compile_ms 942, pipelines/front_ms 912, render/thread/ms 41.40, render/busy/ms 38.56, main/busy/ms 34.15, pipelines/driver_ms 29.62, stage/main/update 23.38, stream/drive/ms 19.27, stage/render/prepare_assets 19.20, stage/render/render 14.62, render/graph/ms 11.56, encode//sum 7.49 |
| warm-up | 4.08 | 46.0 | pipelines/compile_ms 872, pipelines/front_ms 841, render/thread/ms 67.47, render/busy/ms 64.38, main/busy/ms 46.30, stage/render/prepare_assets 37.18, pipelines/driver_ms 30.80, stage/render/render 18.82, stage/main/update 18.57, stage/main/post_update 15.37, render/graph/ms 15.04, stream/drive/ms 13.99 |
| warm-up | 4.17 | 92.2 | pipelines/compile_ms 370, pipelines/front_ms 356, render/thread/ms 106, render/busy/ms 103, main/busy/ms 68.30, stage/render/prepare_assets 65.27, stage/main/update 35.49, stage/render/render 26.09, render/graph/ms 22.86, stream/drive/ms 21.31, stage/main/post_update 18.89, pipelines/driver_ms 13.84 |
| warm-up | 4.22 | 46.2 | pipelines/compile_ms 394, pipelines/front_ms 382, main/busy/ms 117, render/thread/ms 103, render/busy/ms 101, stage/main/update 96.46, stage/render/prepare_assets 60.40, stream/drive/ms 55.27, stage/render/render 23.41, pipelines/wait_ms 21.34, render/graph/ms 20.43, pipelines/driver_ms 12.53 |
| warm-up | 4.39 | 169.7 | pipelines/compile_ms 2391, pipelines/front_ms 2261, pipelines/driver_ms 130, main/busy/ms 122, render/thread/ms 119, render/busy/ms 117, stage/main/update 99.62, stage/render/prepare_assets 90.98, stream/drive/ms 63.50, pipelines/wait_ms 33.85, stage/main/post_update 11.94, stage/render/render 10.98 |
| warm-up | 4.47 | 86.2 | pipelines/compile_ms 2303, pipelines/front_ms 2165, render/thread/ms 150, render/busy/ms 149, pipelines/driver_ms 138, stage/render/prepare_assets 98.01, main/busy/ms 90.06, pipelines/wait_ms 85.27, stage/main/update 44.70, stream/drive/ms 29.83, stage/main/first 17.56, stage/render/extract_commands 17.42 |
| warm-up | 4.54 | 70.0 | pipelines/compile_ms 223, pipelines/front_ms 206, render/thread/ms 114, render/busy/ms 112, main/busy/ms 97.46, pipelines/wait_ms 70.04, stage/render/prepare_assets 57.81, stage/main/update 50.57, stage/main/post_update 20.91, stage/main/first 17.64, stage/render/extract_commands 17.39, pipelines/driver_ms 17.27 |
| warm-up | 4.60 | 59.4 | pipelines/compile_ms 1209, pipelines/front_ms 1164, render/thread/ms 94.76, render/busy/ms 93.32, main/busy/ms 79.18, stage/render/prepare_assets 67.03, stage/main/update 49.19, pipelines/driver_ms 45.35, pipelines/wait_ms 28.87, stage/main/post_update 22.91, stage/render/render 10.29, encode//sum 10.02 |
| warm-up | 4.64 | 36.5 | pipelines/compile_ms 2488, pipelines/front_ms 2395, pipelines/wait_ms 471, pipelines/driver_ms 93.73, render/thread/ms 74.03, render/busy/ms 72.43, main/busy/ms 44.94, stage/render/prepare_assets 43.31, stage/main/update 23.29, stage/main/post_update 14.90, encode//sum 10.51, stage/render/render 9.86 |
| warm-up | 4.67 | 29.3 | pipelines/compile_ms 1834, pipelines/front_ms 1758, pipelines/wait_ms 799, pipelines/driver_ms 75.81, render/thread/ms 57.63, render/busy/ms 56.10, main/busy/ms 29.81, stage/render/prepare_resources 19.00, stage/render/prepare_assets 15.56, encode//sum 14.07, stage/main/post_update 12.98, stage/render/render 11.34 |
| warm-up | 4.71 | 42.8 | pipelines/compile_ms 555, pipelines/front_ms 528, pipelines/wait_ms 356, render/thread/ms 66.73, render/busy/ms 65.03, main/busy/ms 50.15, stage/main/post_update 31.00, stage/render/prepare_resources 30.46, pipelines/driver_ms 27.42, encode//sum 15.91, stage/main/update 13.36, stage/render/render 12.87 |
| warm-up | 4.75 | 33.5 | main/busy/ms 101, stage/main/update 71.59, render/thread/ms 65.72, render/busy/ms 64.11, stream/drive/ms 56.72, pipelines/compile_ms 26.36, pipelines/front_ms 25.07, stage/main/post_update 24.14, stage/render/prepare_resources 17.54, encode//sum 12.92, stage/render/prepare_assets 11.42, stage/render/render 10.83 |
| warm-up | 4.81 | 66.3 | pipelines/compile_ms 1341, pipelines/front_ms 1305, pipelines/wait_ms 677, main/busy/ms 111, stage/main/update 99.36, stream/drive/ms 87.81, render/thread/ms 44.53, render/busy/ms 42.80, pipelines/driver_ms 35.94, stage/render/prepare_assets 15.54, encode//sum 11.58, stage/render/render 10.38 |
| warm-up | 4.86 | 45.2 | pipelines/compile_ms 2130, pipelines/front_ms 2079, pipelines/wait_ms 1080, main/busy/ms 56.55, pipelines/driver_ms 50.88, stage/main/update 42.16, stream/drive/ms 35.16, render/thread/ms 34.25, render/busy/ms 32.26, stage/render/prepare_assets 12.10, stage/render/render 11.62, encode//sum 10.68 |
| warm-up | 4.99 | 88.5 | pipelines/compile_ms 137, pipelines/front_ms 132, main/busy/ms 85.31, render/thread/ms 53.69, render/busy/ms 52.04, stage/main/update 50.51, stage/render/prepare_assets 23.47, stage/main/post_update 23.46, stream/drive/ms 14.05, encode//sum 13.09, stage/render/render 11.48, render/graph/ms 9.25 |
| warm-up | 5.05 | 54.8 | pipelines/compile_ms 321, pipelines/front_ms 309, render/thread/ms 81.15, render/busy/ms 79.62, main/busy/ms 57.10, stage/render/prepare_assets 41.36, stage/main/update 38.84, pipelines/wait_ms 25.51, encode//sum 14.50, pipelines/driver_ms 11.92, stage/render/render 11.91, stage/main/post_update 10.68 |
| warm-up | 5.09 | 39.9 | pipelines/compile_ms 1058, pipelines/front_ms 1026, render/thread/ms 77.54, render/busy/ms 75.53, main/busy/ms 50.72, stage/render/prepare_assets 34.04, pipelines/driver_ms 32.18, pipelines/wait_ms 27.41, stage/main/update 26.49, encode//sum 14.87, stage/main/post_update 13.55, stage/render/render 13.54 |
| warm-up | 5.17 | 29.7 | pipelines/compile_ms 272, pipelines/front_ms 260, render/thread/ms 48.37, render/busy/ms 46.64, main/busy/ms 45.18, stage/main/update 22.33, stage/render/render 18.28, stage/main/post_update 17.71, render/graph/ms 16.31, encode//sum 14.67, stage/render/graph_submit 13.11, stage/render/prepare_resources 12.14 |
| warm-up | 5.21 | 48.7 | pipelines/compile_ms 675, pipelines/front_ms 652, render/thread/ms 70.46, render/busy/ms 68.68, main/busy/ms 54.70, stage/main/update 34.72, pipelines/wait_ms 29.25, pipelines/driver_ms 23.15, stage/render/prepare_assets 22.73, encode//sum 21.98, stage/render/render 17.51, render/graph/ms 15.33 |
| warm-up | 5.28 | 45.5 | pipelines/compile_ms 410, pipelines/front_ms 400, pipelines/wait_ms 63.96, render/thread/ms 53.33, render/busy/ms 51.64, main/busy/ms 38.43, stage/render/prepare_assets 24.00, stage/main/update 21.77, encode//sum 14.56, stage/render/render 14.43, stream/drive/ms 12.57, render/graph/ms 12.12 |
| warm-up | 5.34 | 55.2 | pipelines/compile_ms 1294, pipelines/front_ms 1257, render/thread/ms 82.22, render/busy/ms 80.47, pipelines/wait_ms 78.18, main/busy/ms 57.43, stage/main/update 36.88, pipelines/driver_ms 36.59, stage/render/prepare_flush 31.73, stage/render/prepare_assets 21.84, encode//sum 14.87, stage/render/render 13.15 |
| warm-up | 5.38 | 40.3 | pipelines/compile_ms 1556, pipelines/front_ms 1515, render/thread/ms 75.84, render/busy/ms 74.17, pipelines/wait_ms 58.07, main/busy/ms 55.14, pipelines/driver_ms 40.39, stage/main/update 35.96, stage/render/prepare_flush 31.41, stage/render/prepare_assets 21.38, encode//sum 15.10, stage/render/render 12.48 |
| warm-up | 5.45 | 30.6 | pipelines/compile_ms 80.60, pipelines/front_ms 77.99, render/thread/ms 48.27, render/busy/ms 43.41, main/busy/ms 33.75, stage/main/update 19.27, encode//sum 18.34, stage/render/render 17.25, stream/drive/ms 13.29, render/graph/ms 12.32, stream/spawn/ms 11.51, stage/render/graph_submit 9.41 |
| warm-up | 5.55 | 29.6 | render/thread/ms 43.82, render/busy/ms 42.58, main/busy/ms 28.31, stage/render/prepare_assets 21.61, encode//sum 15.72, stage/main/update 11.72, stage/main/post_update 11.59, stage/render/render 10.91, render/graph/ms 9.27, stage/render/graph_submit 6.63, stream/drive/ms 3.75, encode/submit 3.40 |
| warm-up | 5.62 | 29.8 | pipelines/compile_ms 1116, pipelines/front_ms 1086, render/thread/ms 47.99, render/busy/ms 46.06, main/busy/ms 37.98, pipelines/driver_ms 29.95, stage/render/prepare_assets 23.24, stage/main/update 18.52, encode//sum 15.23, stage/render/render 13.46, stage/main/post_update 12.29, render/graph/ms 11.17 |
| warm-up | 5.66 | 33.4 | pipelines/compile_ms 731, pipelines/front_ms 712, render/thread/ms 56.59, render/busy/ms 55.27, stage/render/prepare_assets 35.37, main/busy/ms 30.25, pipelines/driver_ms 18.55, encode//sum 14.40, stage/main/update 14.11, stage/main/post_update 9.81, stage/render/render 9.33, render/graph/ms 7.66 |
| warm-up | 5.72 | 33.0 | pipelines/compile_ms 165, pipelines/front_ms 158, render/thread/ms 52.22, render/busy/ms 50.25, main/busy/ms 31.26, stage/render/prepare_assets 21.40, encode//sum 19.10, stage/render/render 14.72, stage/main/update 13.38, render/graph/ms 12.28, stage/main/post_update 10.00, stage/render/graph_submit 8.92 |
| warm-up | 5.75 | 39.1 | pipelines/compile_ms 145, pipelines/front_ms 142, render/thread/ms 55.17, render/busy/ms 53.93, stage/render/prepare_assets 34.29, main/busy/ms 31.74, stage/main/update 15.23, encode//sum 14.60, stage/main/post_update 9.16, stage/render/render 8.98, render/graph/ms 7.38, stage/render/graph_submit 5.02 |
| warm-up | 5.83 | 27.4 | render/thread/ms 45.52, render/busy/ms 44.49, main/busy/ms 31.54, stage/render/prepare_assets 25.32, stage/main/update 15.90, encode//sum 13.37, stage/main/post_update 10.05, stage/render/render 8.40, render/graph/ms 6.94, stage/render/graph_submit 4.86, stream/drive/ms 3.67, encode/submit 2.56 |
| warm-up | 5.86 | 30.1 | render/thread/ms 48.78, render/busy/ms 47.04, main/busy/ms 36.62, stage/render/prepare_assets 22.20, stage/main/update 18.36, encode//sum 18.24, stage/main/post_update 12.27, stage/render/render 12.11, render/graph/ms 9.68, stage/render/graph_submit 6.60, stream/drive/ms 5.69, encode/submit 3.40 |
| warm-up | 5.89 | 29.6 | render/thread/ms 46.59, render/busy/ms 44.34, main/busy/ms 32.33, encode//sum 22.29, stage/render/prepare_assets 16.95, stage/render/render 15.50, stage/main/post_update 14.23, render/graph/ms 12.35, stage/main/update 11.60, stage/render/graph_submit 8.04, stream/drive/ms 5.43, encode/finish_wall 4.27 |
| warm-up | 5.93 | 41.1 | render/thread/ms 48.33, render/busy/ms 46.65, main/busy/ms 31.24, stage/render/prepare_assets 23.37, encode//sum 18.32, stage/render/render 13.33, stage/main/post_update 13.30, render/graph/ms 11.08, stage/main/update 10.96, stage/render/graph_submit 7.61, encode/submit 3.92, stream/drive/ms 3.90 |
| warm-up | 5.99 | 34.4 | main/busy/ms 44.72, render/thread/ms 33.14, render/busy/ms 31.51, stage/main/update 26.78, encode//sum 18.42, stream/drive/ms 15.14, stage/main/post_update 12.26, stage/render/render 12.22, stage/render/prepare_assets 11.50, render/graph/ms 10.10, pipelines/compile_ms 9.60, pipelines/front_ms 8.69 |
| warm-up | 6.14 | 29.3 | main/busy/ms 34.43, render/thread/ms 27.74, render/busy/ms 26.43, stage/main/update 22.04, encode//sum 17.48, stream/drive/ms 15.39, stage/render/render 10.72, stage/render/prepare_assets 9.79, render/graph/ms 9.17, stage/main/post_update 8.30, stage/render/graph_submit 6.81, encode/finish_wall 3.73 |
| warm-up | 6.21 | 28.6 | main/busy/ms 33.41, render/thread/ms 28.55, render/busy/ms 27.09, encode//sum 18.05, stage/main/update 16.78, stage/render/render 13.24, stage/main/post_update 11.64, render/graph/ms 11.34, stage/render/prepare_assets 10.15, stage/render/graph_submit 8.35, encode/submit 4.72, encode/finish_wall 3.73 |
| warm-up | 6.28 | 27.4 | render/thread/ms 41.24, render/busy/ms 39.99, stage/render/prepare_assets 22.28, main/busy/ms 20.66, encode//sum 17.20, stage/render/render 10.66, stage/main/post_update 9.19, render/graph/ms 9.12, stage/render/graph_submit 6.91, stage/main/update 6.78, encode/submit 3.48, encode/finish_wall 3.48 |
| warm-up | 6.43 | 28.7 | render/thread/ms 43.48, main/busy/ms 43.41, render/busy/ms 41.52, encode//sum 30.11, stage/render/render 20.86, stage/main/post_update 20.17, render/graph/ms 18.31, stage/main/update 17.66, stage/render/graph_submit 14.21, stage/render/prepare_assets 12.13, stream/drive/ms 9.65, encode/submit 7.40 |
| warm-up | 6.47 | 40.4 | render/thread/ms 51.64, render/busy/ms 50.26, main/busy/ms 43.09, stage/render/prepare_assets 28.82, encode//sum 23.12, stage/main/update 19.54, stage/main/post_update 17.55, stage/render/render 13.67, render/graph/ms 11.75, stream/drive/ms 10.42, stage/render/graph_submit 8.74, encode/finish_wall 5.51 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_nohold\balmora@2560x1440_off=cascadehold-3 --bench-baseline none --off cascadehold
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.1 W · 73 °C (max 76) · VRAM 4955 MiB · throttled 100% of samples
