# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadeencoders · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1458 frames over 15.0 s · **median 10.15 ms (98 fps)** · mean 10.29 · p95 11.74 · p99 12.53 (1% low 80 fps) · worst 53.13 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 280 frames over 8.6 s, worst 2639.3 ms
- hitches (> 25.4 ms): **1** measured, 55 in warm-up
- timed GPU passes: 8.85 ms of a 10.29 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.15 ms, limit 11 ms
- **FAIL** worst frame 53.13 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.883 | 2.903 | 3.363 | 0.99 | 0.060 |
| dlss_super_resolution | 2.070 | 2.084 | 2.394 | 0.99 | 0.180 |
| early prepass | 0.718 | 0.723 | 1.024 | 0.99 | 0.088 |
| atmosphere_luts | 0.444 | 0.447 | 0.729 | 0.99 | 0.003 |
| bloom | 0.319 | 0.321 | 0.327 | 0.99 | 0.065 |
| main_transmissive_pass_3d | 0.271 | 0.273 | 0.296 | 0.99 | 0.008 |
| water_velocity | 0.248 | 0.250 | 0.272 | 0.99 | 0.001 |
| ssao | 0.223 | 0.224 | 0.555 | 0.99 | 0.007 |
| water_sspr | 0.217 | 0.219 | 0.217 | 0.99 | 0.002 |
| atmos_froxel_inject | 0.152 | 0.153 | 0.155 | 0.99 | 0.001 |
| clouds_trace | 0.149 | 0.150 | 0.173 | 0.99 | 0.001 |
| water_fft | 0.146 | 0.147 | 0.143 | 0.99 | 0.003 |
| uniform_allocation | 0.123 | 0.124 | 0.154 | 0.99 | 0.023 |
| upscaling | 0.118 | 0.119 | 0.126 | 0.99 | 0.004 |
| clouds_composite | 0.115 | 0.116 | 0.116 | 0.99 | 0.002 |
| bin_unpacking | 0.096 | 0.097 | 0.115 | 0.99 | 0.016 |
| tonemapping | 0.089 | 0.090 | 0.091 | 0.99 | 0.003 |
| clustering | 0.068 | 0.068 | 0.077 | 0.99 | 0.022 |
| render_sky | 0.054 | 0.054 | 0.055 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.419 | 0.418 | 0.12 | 0.001 |
| clouds_resolve | 0.051 | 0.051 | 0.052 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| early_mesh_preprocessing | 0.031 | 0.031 | 0.037 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.228 | 0.237 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.029 | 0.99 | 0.001 |
| main_transparent_pass_3d | 0.027 | 0.027 | 0.034 | 0.99 | 0.073 |
| water_mips | 0.026 | 0.026 | 0.026 | 0.99 | 0.002 |
| water_snapshot | 0.025 | 0.025 | 0.030 | 0.99 | 0.002 |
| atmosphere_environment | 0.014 | 0.014 | 0.014 | 0.99 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.005 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.350 | 10.778 | 52.24 | 13631.9 |
| render/busy/ms | 7.539 | 8.602 | 50.73 | 10992.3 |
| main/busy/ms | 7.228 | 8.552 | 10.85 | 10538.2 |
| encode//sum | 7.184 | 9.347 | 53.80 | 10473.9 |
| stage/render/render | 5.338 | 6.222 | 48.64 | 7782.9 |
| render/graph/ms | 4.631 | 5.486 | 47.72 | 6751.5 |
| stage/main/post_update | 4.319 | 5.516 | 7.64 | 6297.0 |
| stage/render/graph_submit | 3.629 | 4.445 | 46.82 | 5291.4 |
| encode/submit | 1.858 | 2.401 | 7.58 | 2708.7 |
| stage/main/update | 1.813 | 2.196 | 3.76 | 2643.2 |
| encode/finish_wall | 1.812 | 2.218 | 43.71 | 2641.6 |
| stage/render/prepare_views | 1.378 | 2.871 | 4.25 | 2009.1 |
| render/acquire/ms | 1.332 | 2.819 | 4.21 | 1941.9 |
| stage/render/camera_driver | 0.940 | 1.128 | 1.44 | 1371.2 |
| stage/render/prepare_write_phase_buffers | 0.585 | 0.765 | 1.16 | 853.5 |
| render/present/ms | 0.479 | 0.581 | 4.02 | 697.7 |
| stage/render/prepare_assets | 0.431 | 0.569 | 1.61 | 628.2 |
| stage/render/prepare_resources | 0.395 | 0.479 | 0.66 | 576.2 |
| bigspace/total/ms | 0.374 | 0.792 | 2.63 | 545.9 |
| stage/main/pre_update | 0.317 | 0.437 | 0.62 | 461.9 |
| stage/main/first | 0.287 | 0.367 | 1.52 | 419.0 |
| terrain/update/ms | 0.233 | 0.298 | 0.52 | 340.2 |
| stage/render/prepare_bind_groups | 0.209 | 0.297 | 0.72 | 304.4 |
| terrain/select/ms | 0.202 | 0.257 | 0.49 | 295.1 |
| stage/render/prepare_meshes | 0.197 | 0.258 | 0.51 | 287.4 |
| stage/main/fixed_main_loop | 0.191 | 0.367 | 0.98 | 278.0 |
| stream/drive/ms | 0.186 | 0.264 | 0.42 | 271.6 |
| stage/render/prepare_flush | 0.176 | 0.346 | 0.87 | 256.2 |
| stage/render/extract_commands | 0.174 | 0.219 | 0.32 | 253.6 |
| stage/render/prepare_batch_phases | 0.164 | 0.227 | 0.44 | 239.6 |
| far/select/ms | 0.160 | 0.233 | 0.47 | 233.6 |
| stream/tick/ms | 0.158 | 0.229 | 0.38 | 230.7 |
| controller/feed/ms | 0.154 | 0.211 | 0.55 | 224.3 |
| stage/render/create_views | 0.154 | 0.202 | 0.46 | 224.1 |
| bigspace/high_precision/ms | 0.140 | 0.349 | 2.41 | 203.4 |
| stage/main/last | 0.117 | 0.177 | 0.44 | 171.2 |
| controller/move/ms | 0.114 | 0.184 | 0.42 | 165.6 |
| anim/evaluate/ms | 0.102 | 0.227 | 2.46 | 148.3 |
| bigspace/low_precision/ms | 0.098 | 0.222 | 2.33 | 142.4 |
| stage/render/specialize | 0.074 | 0.100 | 0.39 | 108.5 |
| stage/render/queue | 0.065 | 0.092 | 0.30 | 94.1 |
| stage/render/phase_sort | 0.063 | 0.084 | 0.21 | 91.3 |
| anim/evaluate_cpu/ms | 0.058 | 0.074 | 0.30 | 84.1 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.080 | 0.38 | 68.8 |
| encode/resolve_diagnostics/sum | 0.040 | 0.071 | 0.28 | 58.8 |
| physics/step/ms | 0.030 | 0.057 | 0.18 | 43.1 |
| stage/render/cleanup | 0.025 | 0.039 | 0.34 | 36.3 |
| weather/map/ms | 0.025 | 0.032 | 0.09 | 36.0 |
| encode/camera_driver_span_begin/sum | 0.020 | 0.035 | 0.17 | 28.9 |
| encode/camera_driver_span_end/sum | 0.016 | 0.032 | 0.20 | 23.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.11 | 15.8 |
| script/update/ms | 0.006 | 0.009 | 0.08 | 9.4 |
| controller/avoid/ms | 0.006 | 0.008 | 0.15 | 8.9 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 7.1 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 7.1 |
| anim/apply/ms | 0.004 | 0.006 | 0.09 | 6.3 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.11 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.08 | 1.0 |
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
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.51 | 43.66 | 1.50 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.17 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.20 | 0.01 |
| encode/finish_critical | ms | 1.51 | 43.66 | 1.50 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.28 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.49 | 50.73 | 9.46 |
| gpu/timed/ms | ms | 9.49 | 11.31 | 9.46 |
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
| phases/shadow/batch_sets | count | 2 | 20 | 17 |
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
| preprocess/bind_groups | count | 0 | 39 | 31 |
| preprocess/bind_groups_created | count | 0 | 39 | 31 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 129 | 129 |
| process/cpu_usage | % | 23.56 | 37.02 | 21.15 |
| process/mem_usage | GiB | 4.57 | 4.59 | 4.59 |
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
| system/cpu_usage | % | 29.77 | 51.34 | 27.46 |
| system/mem_usage | % | 65.42 | 65.72 | 65.70 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2544 | 2544 | 2544 |
| textures/images | count | 2009 | 2009 | 2009 |
| views/all/batch_sets | count | 54 | 72 | 69 |
| views/all/bins | count | 5059 | 5366 | 5059 |
| views/all/count | count | 3 | 5 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.0 k | 15.4 k | 15.0 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9498 | 15.5 k | 15.2 k |
| views/cascade/batch_sets | count | 2 | 20 | 17 |
| views/cascade/bins | count | 1887 | 2194 | 1887 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5746 | 6072 | 5746 |
| views/cascade/pipelines | count | 17 | 20 | 17 |
| views/cascade/work_items | count | 36 | 6072 | 5746 |
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
| anim/events/count | count | 0.10 | 3 | 149 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.40 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 31.25 | 33 | 45.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1458 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1458 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1458 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 3 | 901 |
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

## Hitches (> 25.4 ms; slowest 56, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2639.3 | main/busy/ms 2611, stage/main/post_update 2500, stage/main/update 103, surface/advance_cpu_ms 71.70, stage/main/pre_update 3.96, stage/main/last 1.10, surface/cell_upload_ms 1.01, stage/main/fixed_main_loop 0.71, stream/drive/ms 0.55, stage/main/first 0.51, weather/map/ms 0.15, stream/tick/ms 0.10 |
| warm-up | 3.40 | 761.8 | main/busy/ms 2761, stage/main/post_update 2532, render/thread/ms 729, render/busy/ms 560, stage/render/prepare_assets 522, stage/main/update 214, stage/render/prepare_views 166, render/acquire/ms 165, surface/advance_cpu_ms 71.70, stage/render/prepare_resources 23.85, stage/render/render 12.20, encode//sum 9.55 |
| warm-up | 3.47 | 65.5 | render/thread/ms 777, render/busy/ms 606, stage/render/prepare_assets 549, main/busy/ms 173, stage/render/prepare_views 167, render/acquire/ms 165, stage/main/update 115, stage/main/post_update 43.71, stage/render/prepare_resources 26.49, terrain/update/ms 25.51, stage/render/render 24.22, render/graph/ms 17.36 |
| warm-up | 3.53 | 62.4 | pipelines/compile_ms 279, pipelines/front_ms 253, pipelines/wait_ms 98.29, render/thread/ms 85.07, render/busy/ms 77.15, main/busy/ms 49.97, stage/render/render 41.55, stage/main/post_update 32.26, terrain/update/ms 32.06, stage/render/prepare_assets 29.73, render/graph/ms 26.62, pipelines/driver_ms 26.05 |
| warm-up | 3.68 | 128.6 | pipelines/wait_ms 5872, pipelines/compile_ms 1473, pipelines/front_ms 1393, main/busy/ms 137, render/thread/ms 131, render/busy/ms 128, stage/main/post_update 94.08, pipelines/driver_ms 80.73, stage/render/prepare_write_phase_buffers 36.94, stage/main/update 32.41, stage/render/prepare_flush 30.39, stage/render/prepare_assets 28.81 |
| warm-up | 3.73 | 58.0 | pipelines/wait_ms 6029, pipelines/compile_ms 2238, pipelines/front_ms 2127, main/busy/ms 180, render/thread/ms 174, render/busy/ms 172, pipelines/driver_ms 111, stage/main/update 87.01, stage/main/post_update 84.01, stage/render/prepare_assets 46.32, stage/render/prepare_write_phase_buffers 40.93, stage/render/prepare_flush 30.96 |
| warm-up | 3.83 | 94.1 | pipelines/compile_ms 1604, pipelines/front_ms 1516, pipelines/wait_ms 492, render/thread/ms 131, render/busy/ms 128, main/busy/ms 94.06, pipelines/driver_ms 88.53, stage/render/prepare_assets 70.68, stage/main/update 69.08, stage/render/render 35.98, render/graph/ms 32.78, encode//sum 28.31 |
| warm-up | 3.89 | 44.2 | pipelines/compile_ms 812, pipelines/front_ms 781, pipelines/wait_ms 105, render/thread/ms 58.71, render/busy/ms 56.18, main/busy/ms 44.84, pipelines/driver_ms 30.55, stage/render/prepare_assets 26.98, stage/main/post_update 21.70, stage/render/render 14.49, stage/main/update 13.78, encode//sum 11.65 |
| warm-up | 3.95 | 37.2 | pipelines/compile_ms 664, pipelines/front_ms 629, pipelines/wait_ms 259, render/thread/ms 51.62, render/busy/ms 49.77, pipelines/driver_ms 34.88, stage/render/prepare_assets 28.48, main/busy/ms 23.32, stage/render/render 11.78, terrain/update/ms 10.84, stage/main/post_update 10.43, render/graph/ms 9.54 |
| warm-up | 3.99 | 32.4 | pipelines/compile_ms 266, pipelines/front_ms 243, render/thread/ms 66.46, render/busy/ms 63.60, main/busy/ms 55.11, stage/main/update 45.22, stream/drive/ms 41.73, stage/render/prepare_assets 35.81, pipelines/driver_ms 23.03, stage/render/render 16.40, render/graph/ms 10.69, encode//sum 10.14 |
| warm-up | 4.04 | 53.7 | pipelines/compile_ms 951, pipelines/front_ms 906, render/thread/ms 62.78, render/busy/ms 59.94, main/busy/ms 55.08, stage/main/update 45.07, pipelines/driver_ms 44.90, stream/drive/ms 41.33, stage/render/prepare_assets 32.13, stage/render/render 16.32, render/graph/ms 10.76, encode//sum 8.62 |
| warm-up | 4.12 | 59.5 | pipelines/compile_ms 495, pipelines/front_ms 478, render/thread/ms 54.24, render/busy/ms 52.19, main/busy/ms 40.99, stage/render/render 25.08, stage/main/update 24.26, render/graph/ms 22.64, stage/render/prepare_assets 20.93, stream/drive/ms 20.24, stage/render/graph_submit 17.63, pipelines/driver_ms 16.91 |
| warm-up | 4.18 | 59.9 | pipelines/compile_ms 684, pipelines/front_ms 661, render/thread/ms 87.06, render/busy/ms 84.48, stage/render/prepare_assets 45.61, main/busy/ms 40.98, stage/render/render 32.02, render/graph/ms 28.79, stage/main/update 25.98, pipelines/driver_ms 23.34, stream/drive/ms 18.40, stage/render/graph_submit 17.57 |
| warm-up | 4.26 | 86.8 | pipelines/compile_ms 934, pipelines/front_ms 906, render/thread/ms 98.07, render/busy/ms 95.76, main/busy/ms 64.24, stage/render/prepare_assets 56.37, stage/main/update 46.65, stage/render/render 31.55, render/graph/ms 28.34, pipelines/driver_ms 27.77, stream/drive/ms 18.84, stage/render/camera_driver 14.23 |
| warm-up | 4.33 | 71.8 | pipelines/compile_ms 1431, pipelines/front_ms 1389, main/busy/ms 120, stage/main/update 101, render/thread/ms 95.14, render/busy/ms 92.96, stream/drive/ms 65.49, stage/render/prepare_assets 63.91, pipelines/driver_ms 41.97, stage/render/render 21.47, render/graph/ms 18.37, pipelines/wait_ms 10.77 |
| warm-up | 4.45 | 110.6 | pipelines/compile_ms 1782, pipelines/front_ms 1726, main/busy/ms 111, render/thread/ms 90.86, render/busy/ms 88.44, stage/main/update 85.23, stream/drive/ms 66.52, stage/render/prepare_assets 61.73, pipelines/driver_ms 55.37, stage/main/post_update 13.51, stage/render/render 12.91, pipelines/wait_ms 12.44 |
| warm-up | 4.52 | 77.5 | pipelines/compile_ms 1560, pipelines/front_ms 1512, render/thread/ms 106, render/busy/ms 104, main/busy/ms 77.88, stage/render/prepare_assets 59.53, pipelines/driver_ms 48.09, pipelines/wait_ms 41.33, stage/main/update 35.58, stage/main/post_update 29.35, stream/drive/ms 14.12, stage/render/render 11.71 |
| warm-up | 4.65 | 129.2 | pipelines/compile_ms 2202, pipelines/front_ms 2128, pipelines/wait_ms 465, render/thread/ms 173, render/busy/ms 172, main/busy/ms 99.61, stage/render/prepare_assets 90.20, pipelines/driver_ms 74.45, stage/main/update 45.43, stage/main/post_update 45.16, stage/render/prepare_write_phase_buffers 18.23, stream/drive/ms 14.81 |
| warm-up | 4.71 | 57.6 | pipelines/compile_ms 3025, pipelines/front_ms 2916, pipelines/wait_ms 1150, render/thread/ms 152, render/busy/ms 151, pipelines/driver_ms 109, stage/render/prepare_assets 85.43, main/busy/ms 75.63, stage/main/post_update 35.45, stage/main/update 31.08, stage/render/prepare_write_phase_buffers 15.50, stage/render/prepare_resources 12.98 |
| warm-up | 4.74 | 29.7 | pipelines/compile_ms 1411, pipelines/front_ms 1358, pipelines/wait_ms 723, render/thread/ms 61.97, render/busy/ms 60.37, pipelines/driver_ms 52.69, main/busy/ms 43.33, stage/render/prepare_assets 38.63, stage/main/update 18.62, stage/main/post_update 17.49, stage/render/render 10.87, encode//sum 9.92 |
| warm-up | 4.80 | 58.5 | pipelines/compile_ms 295, pipelines/front_ms 285, render/thread/ms 80.42, render/busy/ms 78.85, main/busy/ms 63.77, stage/main/post_update 39.39, pipelines/wait_ms 29.46, stage/render/prepare_resources 26.28, stage/main/update 19.62, stage/render/prepare_write_phase_buffers 14.56, stage/render/prepare_assets 13.72, stage/render/render 10.87 |
| warm-up | 4.83 | 27.8 | pipelines/compile_ms 667, pipelines/front_ms 650, pipelines/wait_ms 294, render/thread/ms 79.72, render/busy/ms 77.73, main/busy/ms 64.11, stage/main/post_update 37.97, stage/render/prepare_resources 28.93, stage/main/update 21.02, pipelines/driver_ms 17.04, stage/render/prepare_write_phase_buffers 14.73, stage/render/render 12.83 |
| warm-up | 4.86 | 29.6 | pipelines/compile_ms 1247, pipelines/front_ms 1215, pipelines/wait_ms 515, main/busy/ms 97.73, stage/main/update 83.86, stream/drive/ms 74.00, render/thread/ms 41.43, render/busy/ms 39.62, pipelines/driver_ms 31.90, stage/render/prepare_assets 16.35, encode//sum 11.27, stage/render/render 10.87 |
| warm-up | 4.95 | 94.9 | pipelines/compile_ms 1794, pipelines/front_ms 1752, pipelines/wait_ms 454, main/busy/ms 95.77, stage/main/update 81.13, stream/drive/ms 74.09, render/thread/ms 43.19, pipelines/driver_ms 42.39, render/busy/ms 39.89, stage/render/prepare_assets 18.99, stage/render/render 15.03, render/graph/ms 11.30 |
| warm-up | 4.98 | 29.1 | pipelines/compile_ms 1091, pipelines/front_ms 1065, pipelines/wait_ms 232, main/busy/ms 53.24, render/thread/ms 48.41, render/busy/ms 44.60, pipelines/lock_ms 27.90, stage/main/update 27.18, stage/render/render 25.60, pipelines/driver_ms 25.28, render/graph/ms 20.86, stage/main/post_update 14.61 |
| warm-up | 5.05 | 72.9 | pipelines/compile_ms 121, pipelines/front_ms 115, main/busy/ms 66.82, render/thread/ms 60.64, render/busy/ms 58.80, stage/main/update 42.52, pipelines/wait_ms 27.25, pipelines/lock_ms 26.60, stage/render/prepare_assets 23.26, stage/render/render 19.97, render/graph/ms 17.13, stage/main/post_update 13.48 |
| warm-up | 5.10 | 50.2 | render/thread/ms 79.74, render/busy/ms 78.27, main/busy/ms 62.47, pipelines/compile_ms 38.31, pipelines/front_ms 36.41, stage/main/update 36.11, stage/render/prepare_assets 33.20, stage/render/prepare_resources 13.75, stage/main/post_update 13.11, encode//sum 12.84, stage/render/render 10.71, stream/drive/ms 10.41 |
| warm-up | 5.15 | 46.6 | pipelines/compile_ms 1026, pipelines/front_ms 992, render/thread/ms 87.28, render/busy/ms 85.80, main/busy/ms 69.57, pipelines/driver_ms 33.41, pipelines/wait_ms 28.82, stage/main/update 26.56, stage/main/post_update 25.48, stage/render/prepare_assets 23.91, stage/render/prepare_resources 21.05, stage/render/prepare_write_phase_buffers 14.30 |
| warm-up | 5.20 | 26.2 | pipelines/compile_ms 1129, pipelines/front_ms 1096, render/thread/ms 42.94, render/busy/ms 41.19, main/busy/ms 37.02, pipelines/driver_ms 33.15, stage/main/update 20.92, pipelines/wait_ms 19.13, stage/render/render 12.62, encode//sum 11.68, render/graph/ms 10.37, stage/main/post_update 9.74 |
| warm-up | 5.26 | 35.5 | pipelines/compile_ms 460, pipelines/front_ms 446, render/thread/ms 44.62, render/busy/ms 42.15, main/busy/ms 40.62, stage/main/update 26.10, stage/render/render 20.70, render/graph/ms 17.96, stream/drive/ms 17.47, encode//sum 14.61, pipelines/driver_ms 14.57, stage/render/graph_submit 12.92 |
| warm-up | 5.28 | 26.6 | pipelines/compile_ms 477, pipelines/front_ms 461, render/thread/ms 49.33, render/busy/ms 47.16, main/busy/ms 36.56, stage/main/update 19.35, stage/render/render 18.22, render/graph/ms 15.77, pipelines/driver_ms 15.36, encode//sum 13.67, stage/render/graph_submit 10.92, stage/main/post_update 10.66 |
| warm-up | 5.33 | 44.3 | pipelines/compile_ms 408, pipelines/front_ms 396, render/thread/ms 54.03, main/busy/ms 53.20, render/busy/ms 52.35, stage/main/post_update 23.18, stage/main/update 22.57, stage/render/prepare_assets 18.31, stage/render/render 16.05, encode//sum 14.66, render/graph/ms 13.90, pipelines/driver_ms 12.19 |
| warm-up | 5.37 | 40.5 | pipelines/compile_ms 545, pipelines/front_ms 521, render/thread/ms 65.93, render/busy/ms 63.67, main/busy/ms 54.39, stage/main/post_update 29.14, pipelines/driver_ms 23.31, stage/render/render 22.29, stage/render/prepare_assets 21.61, render/graph/ms 19.66, stage/main/update 17.84, encode//sum 16.43 |
| warm-up | 5.41 | 38.6 | pipelines/compile_ms 817, pipelines/front_ms 788, render/thread/ms 66.43, render/busy/ms 64.09, main/busy/ms 51.90, pipelines/driver_ms 28.87, stage/main/post_update 23.68, stage/main/update 21.16, stage/render/render 20.12, render/graph/ms 17.48, stage/render/prepare_assets 17.10, stage/render/graph_submit 14.21 |
| warm-up | 5.45 | 46.8 | pipelines/compile_ms 1107, pipelines/front_ms 1079, render/thread/ms 60.88, render/busy/ms 58.92, main/busy/ms 41.73, pipelines/driver_ms 27.92, stage/render/prepare_assets 27.45, stage/main/update 19.56, stage/main/post_update 16.38, stage/render/render 15.51, render/graph/ms 13.08, encode//sum 11.45 |
| warm-up | 5.51 | 27.4 | pipelines/compile_ms 292, pipelines/front_ms 282, render/thread/ms 47.29, render/busy/ms 45.49, main/busy/ms 33.81, stage/render/render 15.85, encode//sum 14.99, render/graph/ms 13.65, stage/main/post_update 10.87, stage/main/fixed_main_loop 10.86, pipelines/driver_ms 10.24, stage/render/graph_submit 9.88 |
| warm-up | 5.55 | 27.7 | pipelines/compile_ms 819, pipelines/front_ms 789, render/thread/ms 45.40, render/busy/ms 43.56, main/busy/ms 37.49, pipelines/driver_ms 29.46, stage/main/update 20.30, encode//sum 13.29, stage/render/render 13.06, stage/main/post_update 12.02, stream/drive/ms 11.16, render/graph/ms 11.00 |
| warm-up | 5.58 | 26.2 | pipelines/compile_ms 602, pipelines/front_ms 586, render/thread/ms 50.47, render/busy/ms 48.54, main/busy/ms 36.47, stage/main/update 20.75, pipelines/driver_ms 15.48, encode//sum 14.43, stage/render/render 12.75, stage/render/prepare_assets 12.31, stream/drive/ms 11.68, render/graph/ms 10.52 |
| warm-up | 5.63 | 30.5 | pipelines/compile_ms 373, pipelines/front_ms 364, render/thread/ms 44.92, render/busy/ms 43.61, main/busy/ms 28.67, stage/render/prepare_assets 21.34, encode//sum 13.44, stage/main/update 11.57, stage/render/render 11.27, stage/main/post_update 10.75, render/graph/ms 9.61, pipelines/driver_ms 8.83 |
| warm-up | 5.69 | 36.3 | pipelines/compile_ms 610, pipelines/front_ms 593, render/thread/ms 51.04, render/busy/ms 49.18, main/busy/ms 43.44, encode//sum 19.61, stage/main/update 19.29, stage/render/prepare_assets 16.78, stage/render/render 16.67, pipelines/driver_ms 16.43, stage/main/post_update 16.34, render/graph/ms 14.05 |
| warm-up | 5.72 | 31.7 | pipelines/compile_ms 1157, pipelines/front_ms 1127, render/thread/ms 55.75, render/busy/ms 54.22, main/busy/ms 31.82, stage/render/prepare_assets 30.93, pipelines/driver_ms 30.07, encode//sum 15.95, stage/main/update 13.67, stage/render/render 12.48, stage/main/post_update 10.53, render/graph/ms 10.33 |
| warm-up | 5.75 | 29.8 | pipelines/compile_ms 547, pipelines/front_ms 533, render/thread/ms 51.76, render/busy/ms 50.32, stage/render/prepare_assets 29.39, main/busy/ms 28.69, pipelines/driver_ms 13.64, encode//sum 13.61, stage/main/update 12.41, stage/render/render 10.26, stage/main/post_update 9.07, render/graph/ms 8.56 |
| warm-up | 5.80 | 35.8 | pipelines/compile_ms 352, pipelines/front_ms 342, render/thread/ms 47.87, render/busy/ms 46.01, main/busy/ms 30.79, stage/render/prepare_assets 20.02, encode//sum 19.03, stage/render/render 14.77, render/graph/ms 12.45, stage/main/update 12.35, stage/main/post_update 11.04, stage/render/graph_submit 9.44 |
| warm-up | 5.84 | 37.5 | render/thread/ms 58.59, render/busy/ms 57.25, main/busy/ms 35.66, stage/render/prepare_assets 35.33, stage/main/update 15.82, encode//sum 14.59, stage/main/post_update 11.38, stage/render/render 11.38, render/graph/ms 9.63, stage/render/graph_submit 7.26, stream/drive/ms 5.76, encode/submit 3.96 |
| warm-up | 5.88 | 35.4 | render/thread/ms 58.18, render/busy/ms 56.89, stage/render/prepare_assets 37.23, main/busy/ms 30.80, stage/main/update 14.60, encode//sum 12.33, stage/main/post_update 10.27, stage/render/render 10.22, render/graph/ms 8.53, stage/render/graph_submit 6.31, stream/drive/ms 5.29, encode/submit 3.29 |
| warm-up | 5.93 | 25.7 | render/thread/ms 41.79, render/busy/ms 40.61, main/busy/ms 32.97, stage/render/prepare_assets 24.67, stage/main/update 17.34, encode//sum 11.69, stage/main/post_update 11.19, stage/render/render 8.57, render/graph/ms 6.73, stream/drive/ms 6.06, stage/render/graph_submit 4.97, encode/submit 2.56 |
| warm-up | 5.95 | 26.4 | render/thread/ms 43.63, render/busy/ms 42.35, main/busy/ms 38.28, stage/main/update 22.54, stage/render/prepare_assets 22.40, encode//sum 12.71, stage/main/post_update 10.42, stage/render/render 9.92, render/graph/ms 8.09, stage/render/graph_submit 6.02, stream/drive/ms 5.29, encode/submit 3.53 |
| warm-up | 5.98 | 32.2 | render/thread/ms 47.05, render/busy/ms 45.05, main/busy/ms 37.83, stage/main/update 18.31, encode//sum 17.76, stage/render/render 16.29, stage/render/prepare_assets 14.57, render/graph/ms 14.04, stage/main/post_update 12.40, stage/render/graph_submit 10.70, encode/submit 7.01, encode/finish_wall 3.90 |
| warm-up | 6.06 | 64.7 | main/busy/ms 58.73, render/thread/ms 38.59, render/busy/ms 37.27, stage/main/update 34.90, stage/main/post_update 17.17, stage/render/prepare_assets 17.07, encode//sum 15.03, stream/drive/ms 13.89, stage/render/render 10.82, render/graph/ms 9.18, stage/render/graph_submit 6.69, encode/finish_wall 3.66 |
| warm-up | 6.10 | 38.0 | render/thread/ms 52.25, render/busy/ms 50.93, main/busy/ms 41.99, stage/render/prepare_assets 30.01, stage/main/update 24.19, encode//sum 15.33, stage/main/post_update 10.94, stage/render/render 10.06, pipelines/compile_ms 9.77, pipelines/front_ms 8.81, render/graph/ms 8.40, stream/drive/ms 7.71 |
| warm-up | 6.23 | 31.1 | pipelines/compile_ms 129, pipelines/front_ms 125, main/busy/ms 37.14, render/thread/ms 30.56, render/busy/ms 29.30, stage/main/update 23.54, encode//sum 16.15, stream/drive/ms 15.47, stage/render/render 11.44, stage/render/prepare_assets 10.36, render/graph/ms 9.78, stage/main/post_update 8.83 |
| warm-up | 6.30 | 27.1 | render/thread/ms 30.86, main/busy/ms 29.69, render/busy/ms 29.42, encode//sum 18.23, stage/main/update 16.84, stage/render/render 14.86, render/graph/ms 12.93, stage/render/graph_submit 9.86, stage/render/prepare_assets 9.73, stage/main/post_update 8.19, encode/submit 5.11, encode/finish_wall 4.89 |
| warm-up | 6.45 | 25.5 | render/thread/ms 32.84, render/busy/ms 31.70, main/busy/ms 23.13, encode//sum 14.60, stage/render/prepare_assets 13.74, stage/render/render 11.12, stage/main/update 10.88, render/graph/ms 9.61, stage/main/post_update 7.55, stage/render/graph_submit 7.39, encode/finish_wall 4.01, stream/drive/ms 3.88 |
| warm-up | 6.50 | 26.1 | render/thread/ms 32.31, main/busy/ms 31.37, render/busy/ms 30.71, encode//sum 24.76, stage/render/render 19.20, render/graph/ms 17.17, stage/main/post_update 14.10, stage/render/graph_submit 14.06, stage/main/update 13.24, encode/submit 7.78, encode/finish_wall 6.43, stage/render/prepare_assets 5.78 |
| warm-up | 6.57 | 47.3 | render/thread/ms 48.14, render/busy/ms 46.56, main/busy/ms 45.74, stage/main/update 24.13, stage/render/prepare_assets 23.71, encode//sum 21.58, stage/render/render 16.61, stage/main/post_update 15.56, render/graph/ms 14.45, stream/drive/ms 11.67, stage/render/graph_submit 11.12, encode/submit 5.91 |
| measure | 5.85 | 53.1 | encode//sum 60.98, render/thread/ms 60.98, render/busy/ms 58.75, stage/render/render 54.28, render/graph/ms 52.74, stage/render/graph_submit 50.78, encode/finish_wall 45.83, main/busy/ms 14.70, stage/main/post_update 8.40, encode/submit 5.03, stage/main/update 3.89, stage/render/camera_driver 1.82 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_noenc\balmora@2560x1440_off=cascadeencoders-3 --bench-baseline none --off cascadeencoders
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.8 W · 74 °C (max 76) · VRAM 5688 MiB · throttled 100% of samples
