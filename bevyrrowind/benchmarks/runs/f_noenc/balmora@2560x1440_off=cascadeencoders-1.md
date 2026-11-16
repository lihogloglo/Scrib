# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadeencoders · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1449 frames over 15.0 s · **median 10.29 ms (97 fps)** · mean 10.35 · p95 11.73 · p99 12.28 (1% low 81 fps) · worst 14.11 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 279 frames over 8.5 s, worst 2656.6 ms
- hitches (> 25.7 ms): **0** measured, 51 in warm-up
- timed GPU passes: 9.02 ms of a 10.35 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.29 ms, limit 11 ms
- pass worst frame 14.11 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.963 | 2.970 | 3.439 | 1.00 | 0.060 |
| dlss_super_resolution | 2.070 | 2.075 | 2.307 | 1.00 | 0.188 |
| early prepass | 0.691 | 0.693 | 1.007 | 1.00 | 0.091 |
| atmosphere_luts | 0.516 | 0.517 | 0.799 | 1.00 | 0.004 |
| bloom | 0.320 | 0.321 | 0.335 | 1.00 | 0.066 |
| main_transmissive_pass_3d | 0.284 | 0.285 | 0.296 | 1.00 | 0.008 |
| water_velocity | 0.247 | 0.248 | 0.267 | 1.00 | 0.001 |
| ssao | 0.244 | 0.244 | 0.550 | 1.00 | 0.007 |
| water_sspr | 0.216 | 0.216 | 0.218 | 1.00 | 0.002 |
| atmos_froxel_inject | 0.155 | 0.155 | 0.162 | 1.00 | 0.001 |
| water_fft | 0.154 | 0.154 | 0.296 | 1.00 | 0.003 |
| clouds_trace | 0.150 | 0.150 | 0.170 | 1.00 | 0.001 |
| uniform_allocation | 0.122 | 0.122 | 0.154 | 1.00 | 0.023 |
| upscaling | 0.116 | 0.117 | 0.125 | 1.00 | 0.004 |
| clouds_composite | 0.114 | 0.114 | 0.117 | 1.00 | 0.002 |
| bin_unpacking | 0.094 | 0.094 | 0.114 | 1.00 | 0.016 |
| tonemapping | 0.085 | 0.086 | 0.087 | 1.00 | 0.003 |
| clustering | 0.065 | 0.066 | 0.075 | 1.00 | 0.023 |
| render_sky | 0.053 | 0.053 | 0.053 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.416 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| water_mips | 0.031 | 0.031 | 0.033 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.037 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.029 | 0.230 | 0.237 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.029 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.030 | 1.00 | 0.073 |
| water_snapshot | 0.026 | 0.026 | 0.031 | 1.00 | 0.002 |
| atmosphere_environment | 0.014 | 0.014 | 0.015 | 1.00 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.017 | 1.00 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.004 | 1.00 | 0.002 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.004 |
| lightprobe_copy | 0.002 | 0.014 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.004 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.420 | 10.771 | 13.24 | 13649.1 |
| encode//sum | 7.836 | 9.921 | 13.51 | 11354.9 |
| render/busy/ms | 7.734 | 8.710 | 11.69 | 11206.6 |
| main/busy/ms | 7.392 | 8.675 | 10.25 | 10710.6 |
| stage/render/render | 5.515 | 6.368 | 8.41 | 7991.1 |
| render/graph/ms | 4.814 | 5.604 | 6.67 | 6974.9 |
| stage/main/post_update | 4.425 | 5.612 | 6.98 | 6411.9 |
| stage/render/graph_submit | 3.788 | 4.545 | 5.64 | 5489.5 |
| encode/submit | 1.969 | 2.474 | 3.99 | 2852.7 |
| stage/main/update | 1.874 | 2.257 | 3.77 | 2714.8 |
| encode/finish_wall | 1.861 | 2.310 | 2.94 | 2695.9 |
| stage/render/prepare_views | 1.259 | 2.785 | 3.70 | 1824.7 |
| render/acquire/ms | 1.214 | 2.722 | 3.65 | 1758.9 |
| stage/render/camera_driver | 0.963 | 1.169 | 1.52 | 1394.9 |
| stage/render/prepare_write_phase_buffers | 0.590 | 0.772 | 4.17 | 855.3 |
| render/present/ms | 0.472 | 0.568 | 1.43 | 683.6 |
| stage/render/prepare_assets | 0.442 | 0.578 | 1.59 | 641.1 |
| stage/render/prepare_resources | 0.394 | 0.468 | 0.64 | 570.5 |
| bigspace/total/ms | 0.391 | 0.863 | 2.72 | 566.3 |
| stage/main/pre_update | 0.317 | 0.439 | 0.60 | 459.1 |
| stage/main/first | 0.286 | 0.354 | 1.34 | 413.7 |
| terrain/update/ms | 0.236 | 0.304 | 0.56 | 342.6 |
| stage/render/prepare_bind_groups | 0.208 | 0.287 | 0.65 | 301.4 |
| terrain/select/ms | 0.206 | 0.264 | 0.51 | 298.5 |
| stage/render/prepare_meshes | 0.198 | 0.260 | 0.48 | 286.3 |
| stage/main/fixed_main_loop | 0.188 | 0.348 | 1.13 | 271.9 |
| stream/drive/ms | 0.187 | 0.265 | 0.55 | 270.7 |
| stage/render/prepare_flush | 0.174 | 0.341 | 0.67 | 252.6 |
| stage/render/extract_commands | 0.172 | 0.212 | 0.44 | 249.6 |
| controller/move/ms | 0.171 | 0.325 | 0.59 | 247.5 |
| controller/feed/ms | 0.166 | 0.221 | 0.50 | 240.0 |
| far/select/ms | 0.161 | 0.233 | 0.62 | 233.5 |
| stage/render/prepare_batch_phases | 0.160 | 0.216 | 0.40 | 232.0 |
| stream/tick/ms | 0.159 | 0.230 | 0.49 | 230.5 |
| stage/render/create_views | 0.156 | 0.205 | 0.45 | 226.0 |
| bigspace/high_precision/ms | 0.150 | 0.480 | 2.50 | 217.1 |
| stage/main/last | 0.120 | 0.176 | 0.73 | 173.8 |
| anim/evaluate/ms | 0.110 | 0.260 | 1.90 | 159.3 |
| bigspace/low_precision/ms | 0.100 | 0.211 | 2.35 | 144.3 |
| stage/render/specialize | 0.076 | 0.101 | 0.39 | 109.8 |
| stage/render/queue | 0.066 | 0.090 | 0.40 | 95.9 |
| stage/render/phase_sort | 0.062 | 0.083 | 0.21 | 90.0 |
| anim/evaluate_cpu/ms | 0.059 | 0.077 | 0.26 | 85.3 |
| encode/resolve_diagnostics/sum | 0.057 | 0.093 | 0.27 | 83.0 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.078 | 0.41 | 70.1 |
| physics/step/ms | 0.029 | 0.054 | 0.23 | 41.8 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.044 | 0.17 | 40.5 |
| weather/map/ms | 0.025 | 0.033 | 0.16 | 36.1 |
| stage/render/cleanup | 0.025 | 0.036 | 0.22 | 35.9 |
| encode/camera_driver_span_end/sum | 0.024 | 0.043 | 0.25 | 34.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.13 | 15.9 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 9.8 |
| controller/avoid/ms | 0.006 | 0.007 | 0.13 | 8.5 |
| script/ui/ms | 0.005 | 0.007 | 0.05 | 7.6 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.0 |
| anim/apply/ms | 0.004 | 0.005 | 0.01 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 1.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.9 |
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
| encode//max | ms | 1.54 | 2.47 | 1.44 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.17 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.25 | 0.02 |
| encode/finish_critical | ms | 1.54 | 2.47 | 1.44 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.27 | 0.07 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.60 | 11.69 | 9.49 |
| gpu/timed/ms | ms | 9.60 | 10.99 | 9.49 |
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
| process/cpu_usage | % | 25.53 | 36.53 | 24.04 |
| process/mem_usage | GiB | 4.58 | 4.64 | 4.64 |
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
| system/cpu_usage | % | 33.36 | 42.84 | 31.32 |
| system/mem_usage | % | 66.06 | 66.22 | 66.22 |
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
| anim/events/count | count | 0.11 | 2 | 153 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.42 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 31.25 | 33 | 45.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1449 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1449 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1449 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 1 | 900 |
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

## Hitches (> 25.7 ms; slowest 51, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2656.6 | main/busy/ms 2627, stage/main/post_update 2525, stage/main/update 95.36, surface/advance_cpu_ms 66.69, stage/main/pre_update 2.63, stage/main/last 1.05, surface/cell_upload_ms 0.86, stage/main/first 0.53, stage/main/fixed_main_loop 0.43, stream/drive/ms 0.34, weather/map/ms 0.11, bigspace/total/ms 0.10 |
| warm-up | 3.41 | 757.6 | main/busy/ms 2776, stage/main/post_update 2551, render/thread/ms 722, render/busy/ms 555, stage/render/prepare_assets 533, stage/main/update 212, stage/render/prepare_views 165, render/acquire/ms 164, surface/advance_cpu_ms 66.69, stage/render/render 10.29, encode//sum 8.62, stage/render/prepare_resources 8.01 |
| warm-up | 3.48 | 67.5 | render/thread/ms 773, render/busy/ms 603, stage/render/prepare_assets 557, main/busy/ms 169, stage/render/prepare_views 166, render/acquire/ms 165, stage/main/update 121, pipelines/compile_ms 76.32, pipelines/front_ms 71.52, stage/main/post_update 37.28, stage/render/render 29.08, render/graph/ms 23.06 |
| warm-up | 3.51 | 28.5 | pipelines/compile_ms 178, pipelines/front_ms 167, render/thread/ms 77.97, render/busy/ms 73.37, pipelines/wait_ms 55.44, main/busy/ms 50.18, stage/render/render 39.37, stage/main/post_update 34.20, render/graph/ms 34.14, terrain/update/ms 24.91, stage/render/prepare_assets 24.64, encode//sum 13.39 |
| warm-up | 3.68 | 171.2 | pipelines/wait_ms 4993, pipelines/compile_ms 1341, pipelines/front_ms 1266, render/thread/ms 191, render/busy/ms 184, main/busy/ms 171, stage/main/post_update 138, pipelines/driver_ms 75.66, stage/render/prepare_batch_phases 73.40, stage/render/render 40.28, render/graph/ms 36.43, encode//sum 32.79 |
| warm-up | 3.73 | 49.2 | pipelines/wait_ms 5219, pipelines/compile_ms 2316, pipelines/front_ms 2188, render/thread/ms 212, main/busy/ms 209, render/busy/ms 206, pipelines/driver_ms 128, stage/main/post_update 125, stage/render/prepare_batch_phases 73.06, stage/main/update 55.76, encode//sum 49.69, stage/render/prepare_resources 39.13 |
| warm-up | 3.84 | 112.0 | pipelines/compile_ms 1866, pipelines/front_ms 1766, pipelines/wait_ms 315, render/thread/ms 134, render/busy/ms 106, pipelines/driver_ms 99.22, main/busy/ms 85.02, stage/main/update 60.66, stage/render/prepare_assets 55.96, stage/render/render 39.15, render/present/ms 27.41, encode//sum 21.71 |
| warm-up | 3.87 | 30.1 | pipelines/compile_ms 1423, pipelines/front_ms 1272, pipelines/wait_ms 260, pipelines/driver_ms 151, render/thread/ms 114, render/busy/ms 86.47, stage/render/prepare_assets 65.53, stage/render/render 40.45, main/busy/ms 34.53, render/present/ms 26.93, stage/main/post_update 15.57, stage/main/update 13.40 |
| warm-up | 3.90 | 26.5 | pipelines/compile_ms 1434, pipelines/front_ms 1283, pipelines/wait_ms 473, pipelines/driver_ms 150, render/thread/ms 52.30, render/busy/ms 50.37, stage/render/prepare_assets 29.46, main/busy/ms 28.11, stage/render/render 14.59, stage/main/post_update 13.61, render/graph/ms 12.33, stage/render/graph_submit 10.14 |
| warm-up | 3.99 | 41.8 | pipelines/compile_ms 271, pipelines/front_ms 255, main/busy/ms 60.51, render/thread/ms 55.28, render/busy/ms 53.00, stage/main/update 43.86, stream/drive/ms 39.08, stage/render/prepare_assets 30.59, stage/render/render 16.94, pipelines/driver_ms 16.79, render/graph/ms 14.68, encode//sum 8.02 |
| warm-up | 4.03 | 39.9 | pipelines/compile_ms 799, pipelines/front_ms 765, render/thread/ms 67.61, render/busy/ms 65.37, pipelines/wait_ms 61.31, main/busy/ms 55.51, stage/render/prepare_assets 44.65, stage/main/update 43.71, stream/drive/ms 39.26, pipelines/driver_ms 34.63, stage/render/render 17.11, render/graph/ms 14.94 |
| warm-up | 4.06 | 31.7 | pipelines/compile_ms 643, pipelines/front_ms 621, pipelines/wait_ms 62.33, render/thread/ms 56.93, render/busy/ms 55.10, main/busy/ms 48.06, stage/render/prepare_assets 32.73, stage/main/update 30.32, pipelines/driver_ms 22.83, stream/drive/ms 19.55, stage/render/render 12.34, stage/main/post_update 10.56 |
| warm-up | 4.13 | 67.4 | pipelines/compile_ms 779, pipelines/front_ms 751, render/thread/ms 77.86, render/busy/ms 75.95, main/busy/ms 64.79, stage/render/prepare_assets 49.87, stage/main/update 39.91, pipelines/driver_ms 27.58, stream/drive/ms 25.89, stage/main/post_update 18.49, stage/render/render 13.83, render/graph/ms 11.32 |
| warm-up | 4.17 | 42.5 | pipelines/compile_ms 882, pipelines/front_ms 853, render/thread/ms 81.20, render/busy/ms 79.51, main/busy/ms 78.38, stage/render/prepare_assets 58.54, stage/main/update 56.51, pipelines/driver_ms 29.24, stream/drive/ms 26.92, stage/main/post_update 14.77, stage/render/render 13.51, render/graph/ms 11.29 |
| warm-up | 4.28 | 108.9 | pipelines/compile_ms 1567, pipelines/front_ms 1518, render/thread/ms 97.74, render/busy/ms 95.81, main/busy/ms 75.37, stage/render/prepare_assets 71.86, stage/main/update 49.84, pipelines/driver_ms 48.18, stream/drive/ms 21.35, stage/render/render 17.04, stage/main/post_update 16.74, render/graph/ms 14.45 |
| warm-up | 4.49 | 185.2 | pipelines/compile_ms 1673, pipelines/front_ms 1629, render/thread/ms 103, render/busy/ms 101, main/busy/ms 97.07, stage/render/prepare_assets 83.94, stage/main/update 76.67, stream/drive/ms 57.46, pipelines/driver_ms 43.51, pipelines/wait_ms 17.79, stage/main/post_update 11.12, stage/render/render 10.46 |
| warm-up | 4.57 | 78.4 | pipelines/compile_ms 1670, pipelines/front_ms 1623, render/thread/ms 151, render/busy/ms 150, stage/render/prepare_assets 101, main/busy/ms 81.02, pipelines/driver_ms 47.76, pipelines/wait_ms 37.71, stage/main/post_update 36.12, stage/main/update 27.12, stage/render/prepare_resources 15.90, stream/drive/ms 14.26 |
| warm-up | 4.63 | 63.7 | pipelines/compile_ms 124, pipelines/front_ms 112, render/thread/ms 107, render/busy/ms 105, main/busy/ms 103, stage/main/post_update 48.75, stage/render/prepare_assets 45.88, stage/main/update 37.42, pipelines/wait_ms 26.76, stage/render/prepare_resources 20.57, stream/drive/ms 13.58, pipelines/driver_ms 11.54 |
| warm-up | 4.69 | 61.8 | pipelines/compile_ms 791, pipelines/front_ms 768, render/thread/ms 88.44, render/busy/ms 85.74, main/busy/ms 76.85, stage/render/prepare_assets 52.36, stage/main/update 33.36, stage/main/post_update 27.66, pipelines/driver_ms 22.53, stage/main/fixed_main_loop 12.09, stage/render/render 10.56, encode//sum 9.58 |
| warm-up | 4.73 | 37.3 | pipelines/compile_ms 2799, pipelines/front_ms 2720, pipelines/wait_ms 922, pipelines/driver_ms 79.58, render/thread/ms 76.96, render/busy/ms 73.62, main/busy/ms 58.23, stage/render/prepare_assets 38.08, stage/main/update 22.34, stage/main/post_update 17.58, stage/render/render 13.28, stage/main/fixed_main_loop 13.00 |
| warm-up | 4.77 | 44.7 | pipelines/compile_ms 2371, pipelines/front_ms 2303, pipelines/wait_ms 1066, render/thread/ms 72.47, render/busy/ms 70.48, pipelines/driver_ms 68.17, main/busy/ms 55.28, stage/main/post_update 29.11, stage/render/prepare_assets 27.52, stage/main/update 15.94, stage/render/prepare_resources 15.75, stage/render/render 12.44 |
| warm-up | 4.81 | 36.0 | pipelines/compile_ms 399, pipelines/front_ms 386, pipelines/wait_ms 147, render/thread/ms 73.84, render/busy/ms 72.04, main/busy/ms 61.38, stage/main/post_update 28.84, stage/render/prepare_assets 23.78, stage/render/prepare_resources 18.91, stage/render/render 14.09, stage/main/update 13.61, pipelines/driver_ms 12.89 |
| warm-up | 4.84 | 27.4 | main/busy/ms 122, stage/main/update 93.00, stream/drive/ms 85.06, render/thread/ms 51.65, render/busy/ms 49.94, pipelines/compile_ms 36.52, pipelines/front_ms 34.75, stage/render/prepare_assets 14.56, stage/render/render 14.49, encode//sum 12.38, render/graph/ms 12.23, stage/main/fixed_main_loop 11.80 |
| warm-up | 4.93 | 96.9 | pipelines/compile_ms 1702, pipelines/front_ms 1666, pipelines/wait_ms 1377, main/busy/ms 110, stage/main/update 92.03, stream/drive/ms 85.62, render/thread/ms 37.75, render/busy/ms 35.94, pipelines/driver_ms 35.83, encode//sum 15.87, stage/render/render 15.58, stage/render/prepare_assets 13.71 |
| warm-up | 5.00 | 41.8 | pipelines/compile_ms 552, pipelines/front_ms 533, main/busy/ms 60.60, render/thread/ms 52.25, render/busy/ms 49.66, stage/main/update 24.95, stage/main/post_update 22.86, pipelines/driver_ms 19.28, stage/render/render 17.58, render/graph/ms 14.21, encode//sum 12.85, stage/render/prepare_assets 11.74 |
| warm-up | 5.07 | 71.9 | pipelines/compile_ms 504, pipelines/front_ms 487, main/busy/ms 96.63, render/thread/ms 71.68, render/busy/ms 68.45, stage/main/update 63.34, stage/render/prepare_assets 22.95, stage/render/render 18.22, pipelines/driver_ms 16.60, stage/main/post_update 16.22, render/graph/ms 14.42, encode//sum 13.86 |
| warm-up | 5.14 | 65.8 | pipelines/compile_ms 662, pipelines/front_ms 644, main/busy/ms 96.13, render/thread/ms 84.72, render/busy/ms 82.08, stage/main/update 65.61, stage/render/prepare_assets 38.95, pipelines/driver_ms 18.63, stage/main/post_update 17.64, pipelines/wait_ms 15.88, stage/render/render 13.74, encode//sum 13.06 |
| warm-up | 5.16 | 25.9 | pipelines/compile_ms 1236, pipelines/front_ms 1196, render/thread/ms 70.96, render/busy/ms 69.41, main/busy/ms 62.66, pipelines/driver_ms 39.70, stage/main/update 38.73, stage/render/prepare_assets 22.62, pipelines/wait_ms 20.81, stage/main/post_update 17.33, encode//sum 12.71, stage/render/render 11.89 |
| warm-up | 5.25 | 36.5 | pipelines/compile_ms 998, pipelines/front_ms 966, render/thread/ms 44.90, main/busy/ms 43.19, render/busy/ms 41.13, pipelines/driver_ms 32.15, stage/main/update 26.54, encode//sum 20.07, stage/render/render 19.08, stream/drive/ms 16.79, render/graph/ms 14.98, stage/render/graph_submit 11.62 |
| warm-up | 5.28 | 29.6 | pipelines/compile_ms 849, pipelines/front_ms 821, render/thread/ms 52.60, render/busy/ms 50.56, main/busy/ms 41.36, pipelines/driver_ms 27.75, encode//sum 19.39, stage/main/update 18.75, stage/render/render 16.61, stage/main/post_update 14.62, render/graph/ms 14.35, stage/render/prepare_resources 11.84 |
| warm-up | 5.32 | 43.4 | pipelines/compile_ms 282, pipelines/front_ms 272, render/thread/ms 58.52, render/busy/ms 56.69, main/busy/ms 44.58, pipelines/wait_ms 39.64, stage/render/prepare_assets 23.89, stage/main/update 21.61, stage/main/post_update 16.42, encode//sum 13.25, stage/render/render 12.74, render/graph/ms 10.47 |
| warm-up | 5.36 | 37.5 | pipelines/compile_ms 934, pipelines/front_ms 901, render/thread/ms 63.62, render/busy/ms 61.69, pipelines/wait_ms 40.14, main/busy/ms 38.98, pipelines/driver_ms 33.18, stage/render/prepare_assets 32.07, stage/main/update 19.08, stage/render/render 14.68, encode//sum 12.79, stage/main/post_update 12.66 |
| warm-up | 5.39 | 31.4 | pipelines/compile_ms 743, pipelines/front_ms 715, render/thread/ms 57.07, render/busy/ms 55.31, main/busy/ms 38.33, pipelines/driver_ms 27.86, stage/render/prepare_assets 25.73, stage/main/update 16.44, stage/main/post_update 14.59, stage/render/render 13.54, encode//sum 12.09, render/graph/ms 11.17 |
| warm-up | 5.42 | 34.9 | pipelines/compile_ms 307, pipelines/front_ms 298, render/thread/ms 51.71, render/busy/ms 50.16, main/busy/ms 35.75, stage/render/prepare_assets 28.24, stage/main/update 15.90, stage/main/post_update 13.27, encode//sum 11.60, stage/render/render 11.22, render/graph/ms 9.30, pipelines/driver_ms 9.16 |
| warm-up | 5.45 | 27.7 | pipelines/compile_ms 728, pipelines/front_ms 709, render/thread/ms 46.78, render/busy/ms 45.22, main/busy/ms 26.38, stage/render/prepare_assets 24.22, pipelines/driver_ms 18.55, encode//sum 13.10, stage/main/update 12.71, stage/render/render 12.34, render/graph/ms 10.24, stage/render/graph_submit 7.68 |
| warm-up | 5.55 | 27.5 | pipelines/compile_ms 415, pipelines/front_ms 402, render/thread/ms 49.19, render/busy/ms 47.50, main/busy/ms 34.76, stage/main/update 19.86, encode//sum 16.04, stage/render/render 13.86, pipelines/driver_ms 13.18, render/graph/ms 11.52, stream/drive/ms 10.08, stage/main/post_update 9.98 |
| warm-up | 5.57 | 26.0 | pipelines/compile_ms 596, pipelines/front_ms 577, render/thread/ms 49.96, render/busy/ms 48.16, main/busy/ms 38.76, pipelines/driver_ms 19.08, stage/main/update 18.98, encode//sum 16.08, stage/render/prepare_assets 15.16, stage/main/post_update 14.08, stage/render/render 13.33, render/graph/ms 10.89 |
| warm-up | 5.62 | 28.8 | pipelines/compile_ms 584, pipelines/front_ms 568, render/thread/ms 44.15, render/busy/ms 42.42, main/busy/ms 32.77, stage/render/prepare_assets 17.14, pipelines/driver_ms 15.46, encode//sum 15.20, stage/main/update 13.76, stage/render/render 13.42, stage/main/post_update 13.00, render/graph/ms 11.23 |
| warm-up | 5.69 | 44.9 | pipelines/compile_ms 1140, pipelines/front_ms 1114, render/thread/ms 59.58, render/busy/ms 58.01, main/busy/ms 41.73, stage/render/prepare_assets 32.07, pipelines/driver_ms 26.09, stage/main/update 20.80, encode//sum 15.73, stage/render/render 14.20, render/graph/ms 11.97, stage/main/post_update 11.72 |
| warm-up | 5.77 | 33.1 | render/thread/ms 52.18, render/busy/ms 50.44, main/busy/ms 34.21, stage/render/prepare_assets 24.19, encode//sum 16.73, stage/main/post_update 13.72, stage/render/render 13.27, stage/main/update 13.00, render/graph/ms 10.87, stage/render/graph_submit 7.65, stream/drive/ms 4.54, encode/submit 4.23 |
| warm-up | 5.80 | 36.1 | pipelines/compile_ms 322, pipelines/front_ms 316, render/thread/ms 55.08, render/busy/ms 53.75, main/busy/ms 34.62, stage/render/prepare_assets 29.38, encode//sum 15.58, stage/main/update 14.20, stage/main/post_update 12.71, stage/render/render 11.45, render/graph/ms 9.60, pipelines/driver_ms 6.95 |
| warm-up | 5.83 | 27.4 | pipelines/compile_ms 322, pipelines/front_ms 316, render/thread/ms 50.02, render/busy/ms 48.81, main/busy/ms 30.63, stage/render/prepare_assets 28.09, encode//sum 15.02, stage/main/update 14.50, stage/render/render 10.61, stage/main/post_update 10.46, render/graph/ms 8.98, pipelines/driver_ms 6.95 |
| warm-up | 5.86 | 29.0 | render/thread/ms 47.78, render/busy/ms 46.63, stage/render/prepare_assets 30.18, main/busy/ms 27.28, stage/main/update 11.93, encode//sum 11.27, stage/main/post_update 9.97, stage/render/render 8.37, render/graph/ms 6.86, stage/render/graph_submit 4.94, stream/drive/ms 4.43, encode/submit 2.57 |
| warm-up | 5.89 | 30.5 | render/thread/ms 50.76, render/busy/ms 49.41, main/busy/ms 36.74, stage/render/prepare_assets 30.01, stage/main/update 16.73, stage/main/post_update 14.09, encode//sum 11.83, stage/render/render 8.62, render/graph/ms 6.89, stage/render/graph_submit 4.86, stream/drive/ms 3.78, encode/submit 2.59 |
| warm-up | 5.92 | 30.7 | render/thread/ms 52.56, render/busy/ms 50.80, main/busy/ms 44.00, stage/render/prepare_assets 25.67, stage/main/update 18.95, stage/main/post_update 18.76, encode//sum 16.19, stage/render/render 12.13, render/graph/ms 9.68, stage/render/graph_submit 6.86, stream/drive/ms 6.26, encode/submit 3.63 |
| warm-up | 5.95 | 30.9 | render/thread/ms 47.36, render/busy/ms 45.50, main/busy/ms 39.40, encode//sum 19.61, stage/main/post_update 18.68, stage/render/prepare_assets 17.93, stage/render/render 15.69, stage/main/update 14.19, render/graph/ms 12.78, stage/render/graph_submit 8.72, stream/drive/ms 6.04, encode/finish_wall 4.66 |
| warm-up | 6.00 | 42.8 | render/thread/ms 50.32, render/busy/ms 48.85, main/busy/ms 42.09, stage/render/prepare_assets 27.48, stage/main/update 19.56, encode//sum 15.65, stage/main/post_update 15.40, stage/render/render 12.23, render/graph/ms 10.03, stage/render/graph_submit 6.87, encode/finish_wall 4.12, stream/drive/ms 3.65 |
| warm-up | 6.03 | 35.3 | render/thread/ms 52.97, render/busy/ms 51.80, main/busy/ms 46.20, stage/render/prepare_assets 33.18, stage/main/update 29.45, stream/drive/ms 14.59, encode//sum 13.24, stage/main/post_update 10.18, stage/render/render 9.98, pipelines/compile_ms 8.40, render/graph/ms 8.34, pipelines/front_ms 7.57 |
| warm-up | 6.18 | 27.9 | main/busy/ms 31.06, render/thread/ms 27.26, render/busy/ms 26.04, stage/main/update 20.17, encode//sum 14.95, stream/drive/ms 13.97, stage/render/render 10.43, stage/render/prepare_assets 9.16, render/graph/ms 8.65, stage/main/post_update 7.15, stage/render/graph_submit 6.19, encode/finish_wall 3.57 |
| warm-up | 6.46 | 26.6 | render/thread/ms 43.47, main/busy/ms 42.64, render/busy/ms 40.92, encode//sum 27.73, stage/main/update 20.99, stage/render/render 20.91, render/graph/ms 18.12, stage/main/post_update 15.56, stage/render/graph_submit 14.10, stream/drive/ms 12.46, stage/render/prepare_assets 10.39, encode/submit 7.30 |
| warm-up | 6.50 | 43.6 | render/thread/ms 49.61, render/busy/ms 47.62, main/busy/ms 46.50, stage/render/prepare_assets 23.97, stage/main/update 22.58, encode//sum 21.33, stage/main/post_update 17.67, stage/render/render 15.76, render/graph/ms 13.63, stream/drive/ms 12.58, stage/render/graph_submit 10.72, encode/submit 5.93 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_noenc\balmora@2560x1440_off=cascadeencoders-1 --bench-baseline none --off cascadeencoders
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.7 W · 75 °C (max 77) · VRAM 5683 MiB · throttled 100% of samples
