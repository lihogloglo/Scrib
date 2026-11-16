# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1409 frames over 15.0 s · **median 10.60 ms (94 fps)** · mean 10.65 · p95 11.59 · p99 12.21 (1% low 82 fps) · worst 14.92 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 280 frames over 8.4 s, worst 2518.7 ms
- hitches (> 26.5 ms): **0** measured, 49 in warm-up
- timed GPU passes: 9.48 ms of a 10.65 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.60 ms, limit 11 ms
- pass worst frame 14.92 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.859 | 2.863 | 2.907 | 1.00 | 0.057 |
| dlss_super_resolution | 2.050 | 2.053 | 2.076 | 1.00 | 0.186 |
| early prepass | 0.808 | 0.809 | 1.014 | 1.00 | 0.085 |
| per_view_shadow_pass_cascade1 | 0.678 | 0.679 | 0.938 | 1.00 | 0.050 |
| atmosphere_luts | 0.350 | 0.350 | 0.422 | 1.00 | 0.003 |
| bloom | 0.318 | 0.319 | 0.335 | 1.00 | 0.065 |
| main_transmissive_pass_3d | 0.278 | 0.278 | 0.295 | 1.00 | 0.008 |
| water_velocity | 0.243 | 0.243 | 0.269 | 1.00 | 0.001 |
| water_sspr | 0.210 | 0.210 | 0.213 | 1.00 | 0.002 |
| ssao | 0.195 | 0.195 | 0.203 | 1.00 | 0.007 |
| uniform_allocation | 0.152 | 0.152 | 0.154 | 1.00 | 0.026 |
| atmos_froxel_inject | 0.151 | 0.151 | 0.157 | 1.00 | 0.001 |
| clouds_trace | 0.151 | 0.151 | 0.171 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.140 | 1.00 | 0.003 |
| upscaling | 0.114 | 0.115 | 0.121 | 1.00 | 0.004 |
| bin_unpacking | 0.114 | 0.114 | 0.115 | 1.00 | 0.020 |
| clouds_composite | 0.113 | 0.113 | 0.116 | 1.00 | 0.002 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.070 | 0.070 | 0.078 | 1.00 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.417 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.052 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.038 | 0.038 | 1.00 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.224 | 0.236 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.029 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.027 | 0.027 | 0.034 | 1.00 | 0.073 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.016 | 0.024 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.008 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.004 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.039 | 0.039 | 0.00 | 0.007 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.017 | 0.017 | 0.00 | 0.005 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.726 | 10.677 | 13.96 | 13703.7 |
| encode//sum | 8.484 | 10.637 | 21.19 | 11953.8 |
| render/busy/ms | 7.580 | 8.459 | 10.58 | 10680.2 |
| main/busy/ms | 7.264 | 8.529 | 9.66 | 10234.8 |
| stage/render/render | 5.272 | 6.125 | 9.58 | 7428.4 |
| render/graph/ms | 4.578 | 5.376 | 7.34 | 6450.9 |
| stage/main/post_update | 4.489 | 5.585 | 6.78 | 6325.0 |
| stage/render/graph_submit | 3.581 | 4.393 | 6.24 | 5045.3 |
| encode/submit | 1.900 | 2.396 | 4.46 | 2676.5 |
| stage/render/prepare_views | 1.722 | 2.621 | 4.20 | 2426.7 |
| encode/finish_wall | 1.719 | 2.142 | 4.12 | 2422.0 |
| stage/main/update | 1.697 | 2.091 | 3.33 | 2390.6 |
| render/acquire/ms | 1.678 | 2.582 | 4.17 | 2364.0 |
| stage/render/camera_driver | 0.940 | 1.087 | 1.33 | 1323.9 |
| stage/render/prepare_write_phase_buffers | 0.712 | 0.860 | 1.27 | 1003.1 |
| render/present/ms | 0.468 | 0.544 | 2.67 | 659.4 |
| stage/render/prepare_assets | 0.434 | 0.557 | 1.51 | 611.3 |
| stage/render/prepare_resources | 0.390 | 0.457 | 0.63 | 550.2 |
| bigspace/total/ms | 0.373 | 0.662 | 2.06 | 525.6 |
| stage/main/pre_update | 0.308 | 0.396 | 0.59 | 433.4 |
| stage/main/first | 0.287 | 0.347 | 1.20 | 405.0 |
| terrain/update/ms | 0.230 | 0.294 | 0.61 | 324.7 |
| terrain/select/ms | 0.201 | 0.256 | 0.58 | 282.7 |
| stage/render/prepare_flush | 0.195 | 0.370 | 0.67 | 275.3 |
| stage/render/prepare_bind_groups | 0.192 | 0.270 | 0.60 | 270.9 |
| stage/main/fixed_main_loop | 0.187 | 0.319 | 0.91 | 264.0 |
| stage/render/prepare_meshes | 0.179 | 0.224 | 0.40 | 252.7 |
| stream/drive/ms | 0.174 | 0.211 | 0.45 | 245.6 |
| stage/render/extract_commands | 0.171 | 0.204 | 0.47 | 241.0 |
| controller/feed/ms | 0.164 | 0.219 | 0.45 | 230.8 |
| stage/render/prepare_batch_phases | 0.163 | 0.217 | 0.33 | 230.3 |
| far/select/ms | 0.151 | 0.189 | 0.35 | 212.5 |
| stream/tick/ms | 0.148 | 0.178 | 0.43 | 208.0 |
| stage/render/create_views | 0.146 | 0.191 | 0.40 | 205.7 |
| bigspace/high_precision/ms | 0.133 | 0.314 | 1.59 | 187.1 |
| controller/move/ms | 0.128 | 0.214 | 0.62 | 180.2 |
| stage/main/last | 0.118 | 0.170 | 0.77 | 166.9 |
| anim/evaluate/ms | 0.112 | 0.263 | 1.53 | 157.2 |
| bigspace/low_precision/ms | 0.095 | 0.212 | 1.75 | 134.0 |
| stage/render/specialize | 0.072 | 0.094 | 0.40 | 100.8 |
| stage/render/queue | 0.065 | 0.088 | 0.32 | 92.2 |
| stage/render/phase_sort | 0.062 | 0.083 | 0.15 | 87.7 |
| anim/evaluate_cpu/ms | 0.060 | 0.084 | 0.24 | 84.7 |
| encode/resolve_diagnostics/sum | 0.052 | 0.081 | 0.14 | 73.8 |
| stage/render/prepare_collect_phase_buffers | 0.050 | 0.083 | 0.36 | 70.2 |
| physics/step/ms | 0.031 | 0.057 | 0.22 | 43.7 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.039 | 0.09 | 35.5 |
| weather/map/ms | 0.024 | 0.031 | 0.21 | 34.3 |
| stage/render/cleanup | 0.024 | 0.032 | 0.18 | 33.7 |
| encode/camera_driver_span_end/sum | 0.021 | 0.036 | 0.09 | 30.1 |
| weather/eval/ms | 0.011 | 0.014 | 0.06 | 15.0 |
| script/update/ms | 0.006 | 0.007 | 0.03 | 8.4 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 8.1 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 6.4 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.2 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 5.8 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| stage/render/prepare | 0.001 | 0.002 | 0.04 | 0.7 |
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
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4212 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.43 | 4.07 | 1.38 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.09 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.09 | 0.02 |
| encode/finish_critical | ms | 1.43 | 4.07 | 1.38 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.14 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.15 | 11.92 | 10.20 |
| gpu/timed/ms | ms | 10.15 | 11.92 | 10.20 |
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
| process/cpu_usage | % | 24.28 | 38.83 | 26.92 |
| process/mem_usage | GiB | 4.56 | 4.63 | 4.63 |
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
| system/cpu_usage | % | 28.89 | 37.68 | 29.34 |
| system/mem_usage | % | 56.22 | 56.43 | 56.43 |
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
| anim/events/count | count | 0.12 | 3 | 164 |
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
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 32.25 | 34 | 45.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1409 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1409 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1409 |
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

## Hitches (> 26.5 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2518.7 | main/busy/ms 2490, stage/main/post_update 2390, stage/main/update 93.96, surface/advance_cpu_ms 66.14, stage/main/pre_update 3.00, stage/main/last 1.00, stream/drive/ms 0.68, surface/cell_upload_ms 0.67, stage/main/first 0.58, stage/main/fixed_main_loop 0.42, stream/tick/ms 0.19, weather/map/ms 0.12 |
| warm-up | 3.23 | 716.2 | main/busy/ms 2629, stage/main/post_update 2419, render/thread/ms 686, render/busy/ms 523, stage/render/prepare_assets 500, stage/main/update 197, stage/render/prepare_views 162, render/acquire/ms 161, surface/advance_cpu_ms 66.14, stage/render/render 10.97, encode//sum 10.15, stage/render/prepare_resources 8.27 |
| warm-up | 3.30 | 65.1 | render/thread/ms 735, render/busy/ms 570, stage/render/prepare_assets 524, stage/render/prepare_views 162, render/acquire/ms 161, main/busy/ms 156, stage/main/update 106, pipelines/compile_ms 68.42, pipelines/front_ms 63.09, stage/main/post_update 39.43, stage/render/render 27.08, terrain/update/ms 22.13 |
| warm-up | 3.34 | 35.8 | pipelines/compile_ms 133, pipelines/front_ms 120, render/thread/ms 81.92, render/busy/ms 71.25, main/busy/ms 51.82, pipelines/wait_ms 44.27, stage/render/render 42.45, stage/main/post_update 34.26, terrain/update/ms 27.09, render/graph/ms 26.94, stage/render/prepare_assets 24.12, pipelines/driver_ms 12.91 |
| warm-up | 3.38 | 47.3 | pipelines/wait_ms 728, pipelines/compile_ms 482, pipelines/front_ms 442, render/thread/ms 79.20, main/busy/ms 69.35, render/busy/ms 69.23, stage/main/post_update 48.01, pipelines/driver_ms 39.30, stage/render/prepare_flush 30.73, stage/render/render 29.33, render/graph/ms 14.63, terrain/update/ms 11.75 |
| warm-up | 3.44 | 54.1 | pipelines/wait_ms 2403, pipelines/compile_ms 686, pipelines/front_ms 634, render/thread/ms 98.85, render/busy/ms 96.74, main/busy/ms 61.46, stage/render/prepare_flush 55.15, pipelines/driver_ms 52.57, stage/main/post_update 33.17, stage/main/update 16.69, stage/render/render 14.01, encode//sum 12.82 |
| warm-up | 3.52 | 87.6 | pipelines/wait_ms 5445, pipelines/compile_ms 1962, pipelines/front_ms 1851, render/thread/ms 139, render/busy/ms 131, pipelines/driver_ms 111, main/busy/ms 66.07, stage/render/prepare_assets 33.11, stage/main/fixed_main_loop 32.48, stage/render/render 30.50, stage/render/prepare_flush 28.76, render/graph/ms 27.89 |
| warm-up | 3.56 | 35.6 | pipelines/wait_ms 3906, pipelines/compile_ms 1780, pipelines/front_ms 1684, main/busy/ms 114, render/thread/ms 113, render/busy/ms 105, pipelines/driver_ms 95.85, stage/main/update 67.86, stage/render/prepare_assets 42.41, stage/main/fixed_main_loop 28.06, stage/render/render 25.40, stage/render/prepare_resources 24.19 |
| warm-up | 3.70 | 136.2 | pipelines/compile_ms 1488, pipelines/front_ms 1319, pipelines/wait_ms 487, pipelines/driver_ms 170, render/thread/ms 111, render/busy/ms 109, main/busy/ms 90.14, stage/main/update 64.59, stage/render/prepare_assets 62.59, stage/render/render 39.11, render/graph/ms 36.65, encode//sum 34.75 |
| warm-up | 3.77 | 30.1 | pipelines/compile_ms 293, pipelines/front_ms 284, render/thread/ms 42.06, render/busy/ms 40.07, main/busy/ms 34.19, stage/main/update 16.76, stage/render/render 16.16, render/graph/ms 13.87, stage/main/post_update 13.18, stage/render/prepare_resources 11.49, encode//sum 10.68, pipelines/driver_ms 8.98 |
| warm-up | 3.81 | 27.9 | render/thread/ms 43.79, pipelines/compile_ms 37.41, pipelines/front_ms 34.65, render/busy/ms 34.18, stage/render/render 22.12, main/busy/ms 14.74, stage/render/prepare_assets 14.62, render/graph/ms 12.04, render/present/ms 9.41, encode//sum 8.05, stage/render/graph_submit 7.72, stage/main/post_update 6.59 |
| warm-up | 3.84 | 29.4 | pipelines/compile_ms 574, pipelines/front_ms 553, render/thread/ms 54.31, main/busy/ms 51.24, render/busy/ms 44.69, stage/main/update 40.15, stream/drive/ms 37.02, stage/render/prepare_assets 28.37, pipelines/driver_ms 21.13, stage/render/render 18.08, render/present/ms 9.29, render/graph/ms 8.02 |
| warm-up | 3.90 | 55.8 | pipelines/compile_ms 1253, pipelines/front_ms 1204, pipelines/wait_ms 96.03, render/thread/ms 61.01, render/busy/ms 59.52, main/busy/ms 51.19, pipelines/driver_ms 49.08, stage/render/prepare_assets 43.89, stage/main/update 41.14, stream/drive/ms 36.95, stage/render/render 9.56, render/graph/ms 7.74 |
| warm-up | 3.96 | 42.2 | pipelines/compile_ms 468, pipelines/front_ms 445, render/thread/ms 57.80, render/busy/ms 55.59, main/busy/ms 38.35, stage/main/update 25.18, stage/render/render 23.97, pipelines/driver_ms 22.82, render/graph/ms 20.96, stream/drive/ms 19.41, stage/render/prepare_assets 17.62, stage/render/graph_submit 16.91 |
| warm-up | 4.02 | 50.7 | pipelines/compile_ms 296, pipelines/front_ms 283, render/thread/ms 70.49, main/busy/ms 68.35, render/busy/ms 67.86, stage/main/update 41.58, stage/render/prepare_assets 32.73, stage/render/render 29.38, render/graph/ms 26.01, stream/drive/ms 20.84, encode//sum 16.74, stage/render/graph_submit 16.73 |
| warm-up | 4.15 | 135.7 | pipelines/compile_ms 962, pipelines/front_ms 924, render/thread/ms 115, render/busy/ms 95.41, main/busy/ms 88.05, stage/render/prepare_assets 69.95, stage/main/update 60.37, pipelines/driver_ms 38.40, stage/render/render 36.40, stream/drive/ms 20.69, render/present/ms 19.29, render/graph/ms 16.27 |
| warm-up | 4.22 | 71.9 | pipelines/compile_ms 2044, pipelines/front_ms 1950, render/thread/ms 132, main/busy/ms 129, render/busy/ms 113, stage/main/update 103, pipelines/driver_ms 94.05, stage/render/prepare_assets 85.39, stream/drive/ms 62.23, stage/render/render 34.94, render/present/ms 18.90, stage/main/post_update 17.12 |
| warm-up | 4.35 | 127.0 | pipelines/compile_ms 2314, pipelines/front_ms 2220, main/busy/ms 132, render/thread/ms 111, render/busy/ms 109, pipelines/driver_ms 93.64, stage/main/update 87.66, stage/render/prepare_assets 80.18, stream/drive/ms 63.60, stage/main/post_update 30.96, pipelines/wait_ms 30.75, stage/render/render 17.99 |
| warm-up | 4.44 | 89.3 | pipelines/compile_ms 2223, pipelines/front_ms 2151, pipelines/wait_ms 154, render/thread/ms 138, render/busy/ms 136, stage/render/prepare_assets 82.93, main/busy/ms 78.42, pipelines/driver_ms 71.46, stage/main/post_update 36.71, stage/main/update 28.04, stage/render/prepare_resources 23.05, stream/drive/ms 14.74 |
| warm-up | 4.52 | 81.6 | pipelines/compile_ms 1775, pipelines/front_ms 1722, pipelines/wait_ms 170, render/thread/ms 142, render/busy/ms 141, main/busy/ms 77.88, stage/render/prepare_assets 72.52, pipelines/driver_ms 52.81, stage/main/update 39.43, stage/render/prepare_resources 34.55, stage/main/post_update 27.73, encode//sum 14.91 |
| warm-up | 4.61 | 86.7 | pipelines/compile_ms 2222, pipelines/front_ms 2163, pipelines/wait_ms 706, render/thread/ms 130, render/busy/ms 128, stage/render/prepare_assets 63.16, main/busy/ms 61.78, pipelines/driver_ms 59.16, stage/main/update 32.26, stage/render/prepare_resources 28.73, stage/main/post_update 20.35, stage/render/render 13.39 |
| warm-up | 4.64 | 31.1 | pipelines/compile_ms 2087, pipelines/front_ms 2028, pipelines/wait_ms 878, render/thread/ms 91.33, render/busy/ms 89.55, pipelines/driver_ms 59.11, main/busy/ms 42.24, stage/render/prepare_assets 35.30, stage/render/prepare_resources 17.62, stage/main/update 17.46, stage/main/post_update 15.69, stage/render/render 13.25 |
| warm-up | 4.68 | 44.3 | pipelines/compile_ms 1033, pipelines/front_ms 1004, pipelines/wait_ms 259, render/thread/ms 68.09, render/busy/ms 65.62, main/busy/ms 47.67, stage/main/post_update 28.64, pipelines/driver_ms 28.58, stage/render/render 22.15, render/graph/ms 19.01, encode//sum 18.10, bigspace/total/ms 16.66 |
| warm-up | 4.71 | 28.9 | pipelines/compile_ms 844, pipelines/front_ms 820, pipelines/wait_ms 114, render/thread/ms 63.95, render/busy/ms 61.59, main/busy/ms 51.71, stage/main/post_update 30.46, pipelines/driver_ms 24.05, stage/render/render 20.05, encode//sum 18.31, bigspace/total/ms 17.88, render/graph/ms 17.20 |
| warm-up | 4.80 | 56.9 | pipelines/compile_ms 664, pipelines/front_ms 648, pipelines/wait_ms 263, main/busy/ms 108, stage/main/update 94.56, stream/drive/ms 88.60, render/thread/ms 32.58, render/busy/ms 31.02, pipelines/driver_ms 16.20, stage/render/prepare_assets 13.78, encode//sum 11.18, stage/render/render 10.57 |
| warm-up | 4.86 | 60.6 | pipelines/compile_ms 843, pipelines/front_ms 825, pipelines/wait_ms 263, main/busy/ms 68.09, stage/main/update 45.97, stream/drive/ms 40.26, render/thread/ms 32.78, render/busy/ms 27.77, pipelines/driver_ms 18.13, stage/render/render 14.40, stage/main/post_update 12.01, encode//sum 10.61 |
| warm-up | 4.91 | 57.6 | pipelines/compile_ms 267, pipelines/front_ms 259, render/thread/ms 67.40, main/busy/ms 64.33, render/busy/ms 62.41, stage/main/update 26.11, stage/main/post_update 22.62, stage/render/prepare_assets 19.79, stage/render/render 17.49, stage/render/prepare_flush 13.51, render/graph/ms 12.21, encode//sum 11.61 |
| warm-up | 4.98 | 64.8 | pipelines/compile_ms 570, pipelines/front_ms 551, render/thread/ms 97.37, render/busy/ms 95.67, main/busy/ms 86.05, stage/main/update 53.88, stage/render/prepare_assets 42.56, stage/main/post_update 20.72, pipelines/driver_ms 19.22, pipelines/wait_ms 17.75, stage/render/render 14.88, stage/render/prepare_flush 13.48 |
| warm-up | 5.03 | 49.8 | pipelines/compile_ms 1127, pipelines/front_ms 1091, render/thread/ms 87.33, render/busy/ms 85.65, main/busy/ms 61.38, stage/render/prepare_assets 47.07, stage/main/update 39.80, pipelines/driver_ms 35.59, stage/render/render 14.64, pipelines/wait_ms 13.45, encode//sum 13.06, stage/main/post_update 12.79 |
| warm-up | 5.06 | 33.3 | pipelines/compile_ms 1086, pipelines/front_ms 1052, render/thread/ms 70.73, render/busy/ms 68.95, main/busy/ms 48.44, pipelines/driver_ms 33.70, stage/render/prepare_assets 23.81, stage/main/update 19.87, stage/render/render 16.88, render/graph/ms 14.45, encode//sum 14.00, stage/main/post_update 12.33 |
| warm-up | 5.16 | 29.7 | pipelines/compile_ms 561, pipelines/front_ms 534, render/thread/ms 44.33, render/busy/ms 42.21, main/busy/ms 35.86, pipelines/driver_ms 26.56, pipelines/wait_ms 24.76, stage/main/update 15.92, stage/main/post_update 14.65, encode//sum 13.37, stage/render/render 12.84, stage/render/prepare_resources 11.99 |
| warm-up | 5.24 | 49.2 | pipelines/compile_ms 724, pipelines/front_ms 703, main/busy/ms 61.86, render/thread/ms 48.42, render/busy/ms 46.86, stage/main/update 29.87, stage/main/post_update 26.13, encode//sum 21.11, pipelines/driver_ms 21.00, stage/render/prepare_assets 20.92, stream/drive/ms 18.09, stage/render/render 14.60 |
| warm-up | 5.29 | 47.0 | pipelines/compile_ms 724, pipelines/front_ms 703, render/thread/ms 63.74, render/busy/ms 62.22, main/busy/ms 51.07, stage/render/prepare_assets 31.71, stage/main/post_update 24.22, pipelines/driver_ms 21.00, stage/main/update 20.90, encode//sum 15.63, stage/render/render 13.46, stream/drive/ms 13.30 |
| warm-up | 5.32 | 32.7 | pipelines/compile_ms 843, pipelines/front_ms 822, render/thread/ms 61.36, render/busy/ms 59.67, main/busy/ms 42.04, stage/render/prepare_assets 23.94, stage/main/post_update 21.96, pipelines/driver_ms 21.24, stage/render/render 17.14, render/graph/ms 14.68, encode//sum 13.30, stage/main/update 12.88 |
| warm-up | 5.35 | 28.7 | pipelines/compile_ms 1139, pipelines/front_ms 1109, render/thread/ms 47.94, render/busy/ms 46.41, main/busy/ms 32.58, pipelines/driver_ms 30.24, stage/render/prepare_assets 19.54, stage/main/update 15.32, stage/render/render 15.11, encode//sum 13.96, render/graph/ms 13.02, stage/main/post_update 10.90 |
| warm-up | 5.37 | 28.3 | pipelines/compile_ms 753, pipelines/front_ms 733, render/thread/ms 42.74, render/busy/ms 41.42, main/busy/ms 28.34, stage/render/prepare_assets 22.80, pipelines/driver_ms 19.69, encode//sum 14.90, stage/main/update 14.65, stage/render/render 10.42, render/graph/ms 8.65, stage/main/post_update 8.62 |
| warm-up | 5.46 | 26.7 | pipelines/compile_ms 195, pipelines/front_ms 190, render/thread/ms 44.52, render/busy/ms 42.93, main/busy/ms 28.76, encode//sum 18.24, stage/render/prepare_assets 15.80, stage/render/render 13.84, stage/main/update 13.74, render/graph/ms 11.73, stage/main/post_update 9.66, stage/render/graph_submit 8.62 |
| warm-up | 5.52 | 32.5 | pipelines/compile_ms 710, pipelines/front_ms 693, render/thread/ms 51.49, render/busy/ms 50.01, main/busy/ms 32.31, stage/render/prepare_assets 22.30, encode//sum 18.92, pipelines/driver_ms 16.86, stage/render/render 13.87, stage/main/update 13.40, stage/main/post_update 11.94, render/graph/ms 11.70 |
| warm-up | 5.58 | 36.1 | pipelines/compile_ms 347, pipelines/front_ms 338, render/thread/ms 52.68, render/busy/ms 51.13, main/busy/ms 31.94, stage/render/prepare_assets 21.80, encode//sum 17.77, stage/main/update 13.39, stage/render/render 12.55, stage/main/post_update 11.61, render/graph/ms 10.38, pipelines/driver_ms 8.29 |
| warm-up | 5.62 | 39.2 | pipelines/compile_ms 320, pipelines/front_ms 312, render/thread/ms 61.04, render/busy/ms 59.91, stage/render/prepare_assets 37.38, main/busy/ms 33.26, stage/main/update 15.61, encode//sum 14.45, stage/main/post_update 10.59, stage/render/render 9.15, render/graph/ms 7.46, pipelines/driver_ms 7.20 |
| warm-up | 5.65 | 27.3 | pipelines/compile_ms 320, pipelines/front_ms 312, render/thread/ms 52.55, render/busy/ms 51.43, stage/render/prepare_assets 34.83, main/busy/ms 29.45, stage/main/update 13.62, encode//sum 12.79, stage/main/post_update 9.64, stage/render/render 8.55, pipelines/driver_ms 7.20, render/graph/ms 7.08 |
| warm-up | 5.67 | 28.8 | render/thread/ms 48.16, render/busy/ms 46.97, main/busy/ms 30.14, stage/render/prepare_assets 28.19, encode//sum 17.04, stage/main/update 13.59, stage/render/render 10.44, stage/main/post_update 10.32, render/graph/ms 8.81, stage/render/graph_submit 6.36, stream/drive/ms 4.65, encode/finish_wall 3.25 |
| warm-up | 5.73 | 36.5 | render/thread/ms 49.75, render/busy/ms 47.89, main/busy/ms 32.81, stage/render/prepare_assets 22.92, encode//sum 19.40, stage/main/update 17.68, stage/render/render 15.54, render/graph/ms 13.00, stage/render/graph_submit 9.96, stage/main/post_update 9.23, stream/drive/ms 6.84, encode/submit 6.49 |
| warm-up | 5.78 | 43.6 | render/thread/ms 66.36, render/busy/ms 64.75, main/busy/ms 38.76, stage/render/prepare_assets 36.34, encode//sum 24.69, stage/render/render 16.45, stage/main/update 15.82, render/graph/ms 13.92, stage/main/post_update 13.65, stage/render/graph_submit 10.16, stream/drive/ms 7.82, encode/submit 5.77 |
| warm-up | 5.82 | 45.9 | render/thread/ms 65.99, render/busy/ms 64.75, stage/render/prepare_assets 43.01, main/busy/ms 38.59, encode//sum 20.99, stage/main/post_update 15.42, stage/main/update 14.37, stage/render/render 12.25, render/graph/ms 10.41, stage/render/graph_submit 7.57, stream/drive/ms 4.32, encode/submit 3.83 |
| warm-up | 5.86 | 36.0 | render/thread/ms 57.73, render/busy/ms 56.38, main/busy/ms 44.19, stage/render/prepare_assets 34.61, stage/main/update 27.22, encode//sum 19.15, stream/drive/ms 13.60, stage/render/render 12.42, stage/main/post_update 11.32, render/graph/ms 10.39, pipelines/compile_ms 8.17, stage/render/graph_submit 7.66 |
| warm-up | 6.03 | 30.4 | render/thread/ms 35.22, render/busy/ms 34.00, main/busy/ms 32.83, stage/main/update 18.97, encode//sum 17.82, stage/render/prepare_assets 14.40, stream/drive/ms 14.05, stage/render/render 11.76, stage/main/post_update 10.14, render/graph/ms 10, stage/render/graph_submit 7.10, encode/finish_wall 3.70 |
| warm-up | 6.30 | 34.0 | render/thread/ms 48.65, render/busy/ms 46.68, main/busy/ms 38.04, encode//sum 23.67, stage/render/prepare_assets 22.22, stage/main/post_update 17.71, stage/render/render 15.61, stage/main/update 13.74, render/graph/ms 13.04, stage/render/graph_submit 9.58, encode/submit 5.36, encode/finish_wall 4.36 |
| warm-up | 6.36 | 37.1 | render/thread/ms 39.10, render/busy/ms 38.09, main/busy/ms 32.27, stage/render/prepare_assets 24.46, stage/main/update 16.46, encode//sum 13.14, stage/main/post_update 9.72, stream/drive/ms 9.33, stage/render/render 7.73, render/graph/ms 6.44, stage/render/graph_submit 4.58, encode/finish_wall 2.59 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_nohold\balmora@2560x1440_off=cascadehold-1 --bench-baseline none --off cascadehold
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 16%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.7 W · 74 °C (max 76) · VRAM 4949 MiB · throttled 100% of samples
