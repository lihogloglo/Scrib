# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1495 frames over 15.0 s · **median 9.96 ms (100 fps)** · mean 10.04 · p95 11.38 · p99 12.03 (1% low 83 fps) · worst 16.61 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 324 frames over 8.7 s, worst 2510.8 ms
- hitches (> 25.0 ms): **0** measured, 35 in warm-up
- timed GPU passes: 8.73 ms of a 10.04 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.96 ms, limit 11 ms
- pass worst frame 16.61 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.474 | 2.501 | 2.562 | 0.99 | 0.070 |
| dlss_super_resolution | 2.034 | 2.056 | 2.078 | 0.99 | 0.180 |
| early prepass | 0.867 | 0.876 | 1.049 | 0.99 | 0.092 |
| atmosphere_luts | 0.386 | 0.391 | 0.683 | 0.99 | 0.003 |
| bloom | 0.316 | 0.319 | 0.336 | 0.99 | 0.064 |
| main_transmissive_pass_3d | 0.277 | 0.280 | 0.296 | 0.99 | 0.008 |
| water_velocity | 0.240 | 0.243 | 0.267 | 0.99 | 0.001 |
| water_sspr | 0.213 | 0.215 | 0.216 | 0.99 | 0.002 |
| ssao | 0.200 | 0.203 | 0.215 | 0.99 | 0.007 |
| per_view_shadow_pass_cascade1 | 0.177 | 0.710 | 0.951 | 0.25 | 0.049 |
| atmos_froxel_inject | 0.151 | 0.152 | 0.159 | 0.99 | 0.001 |
| clouds_trace | 0.148 | 0.150 | 0.170 | 0.99 | 0.001 |
| water_fft | 0.135 | 0.136 | 0.140 | 0.99 | 0.003 |
| uniform_allocation | 0.123 | 0.124 | 0.140 | 0.99 | 0.021 |
| upscaling | 0.114 | 0.115 | 0.122 | 0.99 | 0.004 |
| clouds_composite | 0.113 | 0.114 | 0.116 | 0.99 | 0.002 |
| bin_unpacking | 0.095 | 0.096 | 0.113 | 0.99 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.090 | 0.99 | 0.004 |
| gpu_instance_materialization | 0.078 | 0.079 | 0.092 | 0.99 | 0.004 |
| clustering | 0.070 | 0.071 | 0.079 | 0.99 | 0.022 |
| early_mesh_preprocessing | 0.065 | 0.065 | 0.085 | 0.99 | 0.004 |
| lightprobe_irradiance_map | 0.052 | 0.421 | 0.419 | 0.12 | 0.001 |
| render_sky | 0.052 | 0.053 | 0.053 | 0.99 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.226 | 0.239 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.027 | 0.028 | 0.99 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 0.99 | 0.002 |
| range_unpacking | 0.023 | 0.023 | 0.031 | 0.99 | 0.003 |
| water_mips | 0.023 | 0.023 | 0.023 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.027 | 0.99 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.99 | 0.003 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.017 | 0.99 | 0.041 |
| night_sky | 0.004 | 0.004 | 0.005 | 0.99 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.015 | 0.015 | 0.00 | 0.006 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| per_view_shadow_pass_cascade3 | — | — | — | - | — |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.144 | 10.479 | 15.57 | 13670.2 |
| encode//sum | 7.998 | 10.413 | 15.57 | 11956.5 |
| render/busy/ms | 7.860 | 8.925 | 11.75 | 11751.1 |
| main/busy/ms | 7.226 | 8.418 | 9.77 | 10802.3 |
| stage/render/render | 5.486 | 6.398 | 11.17 | 8201.2 |
| render/graph/ms | 4.783 | 5.614 | 7.63 | 7150.1 |
| stage/main/post_update | 4.126 | 5.290 | 6.20 | 6168.2 |
| stage/render/graph_submit | 3.802 | 4.607 | 6.72 | 5683.3 |
| encode/submit | 1.996 | 2.494 | 7.66 | 2984.3 |
| stage/main/update | 1.878 | 2.221 | 2.71 | 2807.1 |
| encode/finish_wall | 1.839 | 2.294 | 2.86 | 2749.9 |
| stage/render/camera_driver | 0.918 | 1.069 | 1.35 | 1372.9 |
| stage/render/prepare_views | 0.844 | 2.303 | 3.37 | 1262.2 |
| render/acquire/ms | 0.799 | 2.264 | 3.33 | 1194.5 |
| stage/render/prepare_write_phase_buffers | 0.560 | 0.742 | 1.11 | 836.8 |
| render/present/ms | 0.485 | 0.572 | 3.78 | 724.6 |
| stage/render/prepare_assets | 0.443 | 0.572 | 1.74 | 662.1 |
| bigspace/total/ms | 0.410 | 1.094 | 3.06 | 613.0 |
| stage/render/prepare_resources | 0.398 | 0.476 | 0.82 | 594.9 |
| stage/main/pre_update | 0.321 | 0.440 | 0.78 | 480.0 |
| stage/main/first | 0.294 | 0.360 | 1.21 | 438.9 |
| stage/render/prepare_flush | 0.291 | 0.507 | 1.03 | 434.9 |
| stage/render/prepare_bind_groups | 0.260 | 0.369 | 0.67 | 389.3 |
| stage/main/last | 0.240 | 0.292 | 0.67 | 359.4 |
| terrain/update/ms | 0.234 | 0.303 | 0.68 | 350.6 |
| terrain/select/ms | 0.205 | 0.265 | 0.65 | 306.3 |
| stage/render/prepare_batch_phases | 0.200 | 0.282 | 0.44 | 298.7 |
| stream/drive/ms | 0.197 | 0.286 | 0.54 | 294.5 |
| stage/main/fixed_main_loop | 0.185 | 0.348 | 1.06 | 276.8 |
| stage/render/prepare_meshes | 0.184 | 0.240 | 0.62 | 275.6 |
| stage/render/extract_commands | 0.173 | 0.206 | 0.38 | 258.1 |
| bigspace/high_precision/ms | 0.172 | 0.657 | 2.86 | 257.9 |
| far/select/ms | 0.171 | 0.253 | 0.49 | 255.0 |
| stream/tick/ms | 0.168 | 0.250 | 0.50 | 250.9 |
| controller/feed/ms | 0.167 | 0.218 | 0.44 | 250.0 |
| stage/render/create_views | 0.147 | 0.191 | 0.42 | 219.6 |
| controller/move/ms | 0.136 | 0.246 | 0.51 | 203.9 |
| bigspace/low_precision/ms | 0.111 | 0.268 | 1.81 | 165.7 |
| stage/render/queue | 0.098 | 0.129 | 0.36 | 146.8 |
| anim/evaluate/ms | 0.097 | 0.237 | 1.78 | 144.7 |
| stage/render/specialize | 0.093 | 0.124 | 0.44 | 138.3 |
| anim/evaluate_cpu/ms | 0.057 | 0.073 | 0.36 | 85.7 |
| encode/resolve_diagnostics/sum | 0.054 | 0.083 | 0.27 | 81.1 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.080 | 0.32 | 72.2 |
| physics/step/ms | 0.029 | 0.056 | 0.23 | 43.8 |
| weather/map/ms | 0.025 | 0.034 | 0.20 | 37.7 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.040 | 0.20 | 36.5 |
| stage/render/cleanup | 0.023 | 0.034 | 0.34 | 35.0 |
| encode/camera_driver_span_end/sum | 0.021 | 0.036 | 0.34 | 31.9 |
| stage/render/phase_sort | 0.014 | 0.027 | 0.28 | 21.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 16.8 |
| script/update/ms | 0.007 | 0.011 | 0.04 | 10.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.15 | 9.1 |
| script/ui/ms | 0.005 | 0.007 | 0.13 | 8.0 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.3 |
| anim/apply/ms | 0.004 | 0.005 | 0.17 | 6.5 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 1.0 |
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
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.3 k | 37.3 k | 37.3 k |
| encode//max | ms | 1.52 | 2.67 | 1.37 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.20 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.34 | 0.01 |
| encode/finish_critical | ms | 1.52 | 2.67 | 1.37 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.27 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.94 | 11.75 | 10.19 |
| gpu/timed/ms | ms | 9.94 | 11.63 | 10.19 |
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
| phases/opaque3d/bins | count | 878 | 878 | 878 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 878 | 878 | 878 |
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
| process/cpu_usage | % | 26.08 | 42.34 | 33.03 |
| process/mem_usage | GiB | 4.62 | 4.73 | 4.73 |
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
| system/cpu_usage | % | 31.60 | 45.39 | 27.76 |
| system/mem_usage | % | 48.02 | 48.33 | 48.33 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2538 | 2538 | 2538 |
| textures/images | count | 1995 | 1995 | 1995 |
| views/all/batch_sets | count | 65 | 84 | 65 |
| views/all/bins | count | 3111 | 3124 | 3111 |
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
| views/main/bins | count | 2254 | 2254 | 2254 |
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
| anim/events/count | count | 0.11 | 3 | 157 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.32 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 32.50 | 35 | 48.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1495 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1495 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1495 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 1 | 901 |
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

## Hitches (> 25.0 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2510.8 | main/busy/ms 2483, stage/main/post_update 2378, stage/main/update 98.71, surface/advance_cpu_ms 70.95, stage/main/pre_update 2.37, stage/main/last 1.16, surface/cell_upload_ms 0.71, stage/main/first 0.60, stage/main/fixed_main_loop 0.39, stream/drive/ms 0.38, weather/map/ms 0.12, bigspace/total/ms 0.09 |
| warm-up | 3.23 | 719.7 | main/busy/ms 2618, stage/main/post_update 2400, render/thread/ms 688, render/busy/ms 515, stage/render/prepare_assets 478, stage/main/update 206, stage/render/prepare_views 171, render/acquire/ms 170, surface/advance_cpu_ms 70.95, stage/render/prepare_resources 22.39, stage/render/render 10.45, encode//sum 9.56 |
| warm-up | 3.31 | 82.8 | render/thread/ms 755, render/busy/ms 580, stage/render/prepare_assets 522, stage/render/prepare_views 172, render/acquire/ms 171, main/busy/ms 160, stage/main/update 113, stage/main/post_update 36.00, stage/render/prepare_resources 25.58, stage/render/render 24.94, render/graph/ms 19.67, pipelines/compile_ms 18.31 |
| warm-up | 3.35 | 40.6 | pipelines/compile_ms 249, pipelines/front_ms 229, render/thread/ms 90.04, render/busy/ms 79.52, main/busy/ms 48.74, stage/render/prepare_assets 44.42, stage/render/render 30.68, stage/main/post_update 30.19, terrain/update/ms 23.03, pipelines/wait_ms 22.94, pipelines/driver_ms 20.20, render/graph/ms 19.76 |
| warm-up | 3.44 | 88.6 | pipelines/wait_ms 2166, pipelines/compile_ms 962, pipelines/front_ms 898, render/thread/ms 111, render/busy/ms 101, pipelines/driver_ms 63.89, main/busy/ms 63.30, stage/main/post_update 42.32, stage/render/prepare_bind_groups 34.17, stage/render/prepare_resources 28.39, stage/render/render 19.32, terrain/update/ms 13.19 |
| warm-up | 3.55 | 102.8 | pipelines/wait_ms 7518, pipelines/compile_ms 1980, pipelines/front_ms 1867, render/thread/ms 172, render/busy/ms 170, pipelines/driver_ms 113, stage/render/prepare_resources 71.43, main/busy/ms 55.97, stage/render/prepare_assets 36.22, stage/main/post_update 36.10, stage/render/prepare_bind_groups 34.43, terrain/update/ms 16.51 |
| warm-up | 3.59 | 41.6 | pipelines/wait_ms 5724, pipelines/compile_ms 1648, pipelines/front_ms 1568, render/thread/ms 123, render/busy/ms 121, main/busy/ms 81.65, pipelines/driver_ms 79.34, stage/main/update 60.05, stage/render/prepare_resources 48.54, stage/render/prepare_assets 37.81, encode//sum 31.31, stage/render/render 29.60 |
| warm-up | 3.72 | 136.4 | pipelines/compile_ms 2084, pipelines/front_ms 1992, pipelines/wait_ms 671, render/thread/ms 143, render/busy/ms 113, pipelines/driver_ms 91.85, main/busy/ms 89.49, stage/render/prepare_assets 70.46, stage/main/update 65.38, stage/render/render 61.66, render/graph/ms 31.74, render/present/ms 29.48 |
| warm-up | 3.79 | 27.5 | pipelines/compile_ms 326, pipelines/front_ms 309, render/thread/ms 44.65, render/busy/ms 38.87, main/busy/ms 30.54, encode//sum 24.68, stage/render/render 21.72, render/graph/ms 17.98, pipelines/driver_ms 17.61, stage/main/post_update 17.45, stage/render/graph_submit 15.60, encode/submit 12.16 |
| warm-up | 3.95 | 139.6 | pipelines/compile_ms 1140, pipelines/front_ms 1078, main/busy/ms 95.11, render/thread/ms 92.08, render/busy/ms 89.19, stage/main/update 82.01, pipelines/driver_ms 62.21, stage/render/prepare_assets 60.68, stage/render/render 22.45, render/graph/ms 19.04, stage/render/graph_submit 16.27, pipelines/wait_ms 11.08 |
| warm-up | 4.01 | 56.3 | pipelines/compile_ms 1433, pipelines/front_ms 1360, render/thread/ms 98.76, render/busy/ms 96.50, stage/render/prepare_assets 74.45, pipelines/driver_ms 72.65, main/busy/ms 41.45, stage/main/update 29.73, stage/render/render 17.40, render/graph/ms 14.64, pipelines/wait_ms 13.93, stage/render/graph_submit 11.88 |
| warm-up | 4.09 | 85.9 | pipelines/compile_ms 782, pipelines/front_ms 755, render/thread/ms 92.16, render/busy/ms 90.78, stage/render/prepare_assets 78.04, pipelines/driver_ms 26.19, main/busy/ms 16.03, stage/render/render 9.34, render/graph/ms 7.71, encode//sum 6.84, pipelines/wait_ms 5.93, stage/render/graph_submit 5.53 |
| warm-up | 4.14 | 37.5 | pipelines/compile_ms 460, pipelines/front_ms 425, render/thread/ms 42.41, render/busy/ms 36.49, pipelines/driver_ms 34.91, stage/render/render 32.72, render/graph/ms 29.45, main/busy/ms 27.74, stage/render/graph_submit 25.01, encode/submit 22.50, stage/main/update 13.23, stage/main/post_update 10.00 |
| warm-up | 4.22 | 51.8 | pipelines/compile_ms 155, pipelines/front_ms 150, main/busy/ms 73.56, render/thread/ms 40.26, render/busy/ms 36.91, stage/main/update 31.66, stage/main/post_update 28.03, stream/drive/ms 18.56, stage/render/prepare_write_phase_buffers 10.11, stage/render/render 9.69, stage/render/prepare_assets 7.95, stage/main/first 6.97 |
| warm-up | 4.32 | 101.0 | pipelines/compile_ms 1553, pipelines/front_ms 1507, main/busy/ms 174, stage/main/update 147, render/thread/ms 107, render/busy/ms 104, pipelines/driver_ms 45.51, stage/render/prepare_bind_groups 26.83, stage/render/prepare_flush 22.62, stage/render/render 16.90, stage/render/prepare_write_phase_buffers 14.92, stage/render/prepare_assets 14.31 |
| warm-up | 4.49 | 171.8 | pipelines/compile_ms 4003, pipelines/front_ms 3808, pipelines/wait_ms 626, pipelines/driver_ms 195, render/thread/ms 166, render/busy/ms 165, main/busy/ms 163, stage/main/update 145, stage/render/prepare_assets 76.66, stage/render/prepare_bind_groups 27.71, stage/render/prepare_flush 22.53, stage/render/render 15.73 |
| warm-up | 4.58 | 79.4 | pipelines/compile_ms 84.99, main/busy/ms 82.65, pipelines/front_ms 81.21, stage/main/update 62.43, stream/drive/ms 42.50, render/thread/ms 40.77, render/busy/ms 38.36, stage/render/prepare_assets 18.25, encode//sum 12.33, stage/main/post_update 11.66, stage/render/render 11.13, render/graph/ms 8.34 |
| warm-up | 4.62 | 39.7 | pipelines/compile_ms 88.80, pipelines/front_ms 84.61, render/thread/ms 58.57, render/busy/ms 56.02, main/busy/ms 53.55, stage/main/update 35.83, stage/render/prepare_assets 31.07, stage/render/render 12.06, stream/drive/ms 11.34, encode//sum 10.87, stage/main/post_update 9.74, render/graph/ms 9.17 |
| warm-up | 4.69 | 72.0 | pipelines/compile_ms 942, pipelines/front_ms 904, pipelines/wait_ms 245, render/thread/ms 84.16, render/busy/ms 82.49, main/busy/ms 70.84, stage/render/prepare_assets 51.40, pipelines/driver_ms 37.44, stage/main/update 27.26, stage/main/post_update 27.05, encode//sum 10.97, stage/main/fixed_main_loop 10.44 |
| warm-up | 4.75 | 61.9 | pipelines/compile_ms 1045, pipelines/front_ms 1004, pipelines/wait_ms 344, render/thread/ms 111, render/busy/ms 109, main/busy/ms 87.48, stage/render/prepare_assets 46.21, pipelines/driver_ms 41.18, stage/main/update 40.16, stage/main/post_update 28.95, stage/render/prepare_resources 15.78, encode//sum 13.01 |
| warm-up | 4.80 | 47.1 | pipelines/compile_ms 697, pipelines/front_ms 671, pipelines/wait_ms 99.02, render/thread/ms 89.49, render/busy/ms 87.80, main/busy/ms 62.34, stage/render/prepare_assets 33.67, stage/main/update 31.24, pipelines/driver_ms 26.03, stage/main/post_update 22.82, encode//sum 13.05, stage/render/prepare_resources 13.02 |
| warm-up | 4.85 | 49.1 | pipelines/compile_ms 1699, pipelines/front_ms 1648, pipelines/wait_ms 123, render/thread/ms 83.53, render/busy/ms 81.93, main/busy/ms 66.18, stage/main/post_update 51.04, pipelines/driver_ms 50.63, stage/render/prepare_resources 28.02, stage/render/prepare_assets 26.96, encode//sum 12.51, stage/render/render 11.70 |
| warm-up | 4.93 | 61.4 | pipelines/compile_ms 437, pipelines/front_ms 424, pipelines/wait_ms 80.62, main/busy/ms 52.77, render/thread/ms 52.14, render/busy/ms 50.12, stage/main/update 32.37, stage/render/prepare_assets 27.91, stage/render/render 13.12, pipelines/driver_ms 13.04, stream/drive/ms 12.64, encode//sum 11.57 |
| warm-up | 4.98 | 48.5 | pipelines/compile_ms 903, pipelines/front_ms 883, pipelines/wait_ms 88.12, render/thread/ms 64.65, render/busy/ms 63.30, stage/render/prepare_assets 43.05, main/busy/ms 41.59, stage/main/update 21.11, pipelines/driver_ms 20.34, stream/drive/ms 12.67, encode//sum 12.10, stage/render/render 10.58 |
| warm-up | 5.06 | 66.0 | pipelines/compile_ms 935, pipelines/front_ms 904, main/busy/ms 70.48, pipelines/wait_ms 52.13, stage/main/update 51.95, render/thread/ms 44.99, render/busy/ms 43.25, pipelines/driver_ms 30.59, stage/render/prepare_assets 17.40, encode//sum 13.53, stage/render/render 12.36, stream/drive/ms 12.16 |
| warm-up | 5.09 | 35.9 | pipelines/compile_ms 832, pipelines/front_ms 802, render/thread/ms 58.34, render/busy/ms 56.86, main/busy/ms 51.74, stage/main/update 37.49, pipelines/driver_ms 30.35, stage/render/prepare_assets 23.90, encode//sum 14.55, stage/render/render 13.24, render/graph/ms 11.22, pipelines/wait_ms 10.43 |
| warm-up | 5.32 | 34.4 | pipelines/compile_ms 810, pipelines/front_ms 787, render/thread/ms 48.53, render/busy/ms 45.08, main/busy/ms 32.53, pipelines/driver_ms 22.29, stage/render/render 20.49, render/graph/ms 16.70, stage/main/update 15.67, encode//sum 15.67, stage/render/graph_submit 13.03, stage/main/post_update 11.98 |
| warm-up | 5.35 | 29.1 | pipelines/compile_ms 318, pipelines/front_ms 305, render/thread/ms 55.17, render/busy/ms 51.82, main/busy/ms 44.03, stage/main/update 22.47, stage/render/render 20.92, encode//sum 18.65, render/graph/ms 16.86, stage/main/post_update 14.71, stage/render/graph_submit 13.23, pipelines/driver_ms 13.08 |
| warm-up | 5.40 | 50.0 | pipelines/compile_ms 304, pipelines/front_ms 293, render/thread/ms 51.76, render/busy/ms 50.27, main/busy/ms 40.29, stage/render/prepare_assets 21.52, stage/main/update 20.54, encode//sum 17.02, pipelines/wait_ms 14.43, stage/render/render 13.33, stage/main/post_update 12.41, pipelines/driver_ms 11.50 |
| warm-up | 5.58 | 28.6 | render/thread/ms 42.93, render/busy/ms 40.76, main/busy/ms 37.29, encode//sum 21.06, stage/main/update 17.58, stage/render/render 16.48, render/graph/ms 13.93, stage/main/post_update 13.72, stage/render/graph_submit 10.11, stage/render/prepare_assets 8.86, encode/submit 5.43, stage/render/prepare_meshes 5.33 |
| warm-up | 5.63 | 48.8 | render/thread/ms 57.38, render/busy/ms 55.47, main/busy/ms 40.93, stage/render/prepare_assets 28.63, stage/main/update 21.95, encode//sum 20.85, stage/render/render 13.64, render/graph/ms 11.13, stage/main/post_update 10.73, stage/render/graph_submit 8.19, stream/drive/ms 6.16, encode/finish_wall 4.30 |
| warm-up | 5.67 | 40.5 | pipelines/compile_ms 303, pipelines/front_ms 297, render/thread/ms 60.73, render/busy/ms 59.49, main/busy/ms 42.06, stage/render/prepare_assets 40.30, stage/main/update 26.92, encode//sum 14.66, stage/render/render 9.84, stage/main/post_update 8.36, render/graph/ms 8.07, pipelines/driver_ms 6.48 |
| warm-up | 6.15 | 42.2 | render/thread/ms 58.24, render/busy/ms 57.12, stage/render/prepare_bind_groups 30.00, main/busy/ms 23.92, encode//sum 14.06, stage/main/update 12.81, stage/render/prepare_assets 11.35, stage/render/render 9.84, render/graph/ms 8.33, stage/main/post_update 7.38, stage/render/graph_submit 6.05, encode/finish_wall 3.07 |
| warm-up | 6.38 | 32.8 | render/thread/ms 41.08, render/busy/ms 39.24, main/busy/ms 37.65, encode//sum 18.75, stage/main/update 16.28, stage/main/post_update 15.91, stage/render/render 15.59, render/graph/ms 13.38, stage/render/prepare_assets 11.77, stage/render/graph_submit 9.71, encode/submit 5.05, stream/drive/ms 4.74 |
| warm-up | 6.41 | 31.6 | render/thread/ms 43.22, render/busy/ms 42.04, main/busy/ms 30.20, stage/render/prepare_assets 20.87, stage/main/update 14.23, encode//sum 13.62, stage/main/post_update 9.81, stage/render/render 9.34, render/graph/ms 7.88, stage/render/graph_submit 5.75, stage/render/prepare_meshes 5.04, stream/drive/ms 3.98 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\f_scene\balmora@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 96%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.9 W · 74 °C (max 77) · VRAM 4933 MiB · throttled 100% of samples
