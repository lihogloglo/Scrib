# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs,cascadeencoders,cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1425 frames over 15.0 s · **median 10.48 ms (95 fps)** · mean 10.53 · p95 11.63 · p99 12.32 (1% low 81 fps) · worst 16.80 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 280 frames over 8.5 s, worst 2521.5 ms
- hitches (> 26.2 ms): **0** measured, 47 in warm-up
- timed GPU passes: 8.89 ms of a 10.53 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.48 ms, limit 11 ms
- pass worst frame 16.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.837 | 2.847 | 3.065 | 1.00 | 0.056 |
| dlss_super_resolution | 2.052 | 2.059 | 2.077 | 1.00 | 0.185 |
| early prepass | 0.901 | 0.904 | 1.113 | 1.00 | 0.083 |
| atmosphere_luts | 0.358 | 0.360 | 0.517 | 1.00 | 0.003 |
| bloom | 0.317 | 0.318 | 0.335 | 1.00 | 0.065 |
| main_transmissive_pass_3d | 0.271 | 0.272 | 0.296 | 1.00 | 0.008 |
| water_velocity | 0.243 | 0.244 | 0.269 | 1.00 | 0.001 |
| water_sspr | 0.209 | 0.210 | 0.215 | 1.00 | 0.002 |
| ssao | 0.198 | 0.199 | 0.208 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.173 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.148 | 0.148 | 0.154 | 1.00 | 0.001 |
| uniform_allocation | 0.148 | 0.148 | 0.154 | 1.00 | 0.027 |
| water_fft | 0.135 | 0.136 | 0.139 | 1.00 | 0.003 |
| upscaling | 0.117 | 0.118 | 0.124 | 1.00 | 0.004 |
| clouds_composite | 0.114 | 0.114 | 0.116 | 1.00 | 0.002 |
| bin_unpacking | 0.109 | 0.109 | 0.113 | 1.00 | 0.020 |
| tonemapping | 0.086 | 0.086 | 0.093 | 1.00 | 0.003 |
| clustering | 0.068 | 0.069 | 0.083 | 1.00 | 0.022 |
| render_sky | 0.053 | 0.053 | 0.055 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.414 | 0.417 | 0.12 | 0.001 |
| clouds_resolve | 0.051 | 0.051 | 0.053 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.044 | 0.042 | 1.00 | 0.006 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.234 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.027 | 0.027 | 0.035 | 1.00 | 0.073 |
| water_snapshot | 0.025 | 0.025 | 0.027 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.031 | 1.00 | 0.007 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.004 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.599 | 10.673 | 15.92 | 13678.3 |
| encode//sum | 8.758 | 11.011 | 16.18 | 12480.5 |
| render/busy/ms | 8.025 | 9.156 | 13.53 | 11435.0 |
| main/busy/ms | 7.442 | 8.640 | 10.22 | 10604.9 |
| stage/render/render | 5.738 | 6.799 | 11.84 | 8176.8 |
| render/graph/ms | 5.016 | 6.017 | 9.21 | 7147.3 |
| stage/main/post_update | 4.518 | 5.629 | 6.91 | 6437.8 |
| stage/render/graph_submit | 4.014 | 5.005 | 7.89 | 5720.3 |
| encode/submit | 2.121 | 2.778 | 6.69 | 3022.5 |
| encode/finish_wall | 1.938 | 2.425 | 3.16 | 2761.7 |
| stage/main/update | 1.823 | 2.163 | 3.84 | 2598.1 |
| stage/render/prepare_views | 1.126 | 2.248 | 3.67 | 1604.7 |
| render/acquire/ms | 1.082 | 2.200 | 3.61 | 1541.4 |
| stage/render/camera_driver | 0.941 | 1.105 | 1.57 | 1341.6 |
| stage/render/prepare_write_phase_buffers | 0.689 | 0.826 | 1.31 | 982.1 |
| render/present/ms | 0.493 | 0.586 | 3.72 | 701.8 |
| stage/render/prepare_assets | 0.447 | 0.578 | 1.58 | 636.7 |
| bigspace/total/ms | 0.395 | 0.960 | 2.88 | 563.4 |
| stage/render/prepare_resources | 0.347 | 0.442 | 0.75 | 494.3 |
| stage/main/pre_update | 0.315 | 0.427 | 0.72 | 448.7 |
| stage/main/first | 0.298 | 0.364 | 1.31 | 424.3 |
| terrain/update/ms | 0.234 | 0.300 | 0.68 | 333.2 |
| terrain/select/ms | 0.203 | 0.257 | 0.60 | 289.1 |
| stage/render/prepare_bind_groups | 0.203 | 0.290 | 0.63 | 288.9 |
| stream/drive/ms | 0.193 | 0.284 | 0.73 | 274.7 |
| stage/render/prepare_flush | 0.191 | 0.364 | 0.81 | 272.5 |
| stage/main/fixed_main_loop | 0.190 | 0.343 | 0.84 | 271.1 |
| stage/render/prepare_meshes | 0.181 | 0.234 | 0.52 | 257.6 |
| controller/feed/ms | 0.169 | 0.223 | 0.54 | 241.0 |
| stage/render/extract_commands | 0.166 | 0.198 | 0.38 | 236.2 |
| stream/tick/ms | 0.166 | 0.250 | 0.70 | 235.9 |
| far/select/ms | 0.165 | 0.243 | 0.50 | 235.0 |
| stage/render/prepare_batch_phases | 0.164 | 0.225 | 0.40 | 234.1 |
| bigspace/high_precision/ms | 0.156 | 0.497 | 2.60 | 222.2 |
| stage/render/create_views | 0.139 | 0.183 | 0.54 | 198.3 |
| stage/main/last | 0.117 | 0.172 | 0.68 | 167.0 |
| controller/move/ms | 0.103 | 0.190 | 0.44 | 147.3 |
| stage/render/queue | 0.103 | 0.138 | 0.47 | 147.1 |
| bigspace/low_precision/ms | 0.101 | 0.214 | 2.42 | 143.9 |
| anim/evaluate/ms | 0.100 | 0.216 | 2.30 | 142.9 |
| stage/render/specialize | 0.090 | 0.116 | 0.58 | 128.7 |
| stage/render/phase_sort | 0.067 | 0.116 | 0.30 | 95.6 |
| anim/evaluate_cpu/ms | 0.058 | 0.075 | 0.24 | 82.0 |
| encode/resolve_diagnostics/sum | 0.056 | 0.089 | 0.20 | 80.0 |
| stage/render/prepare_collect_phase_buffers | 0.050 | 0.077 | 0.38 | 70.6 |
| physics/step/ms | 0.029 | 0.055 | 0.24 | 41.6 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.043 | 0.19 | 38.9 |
| weather/map/ms | 0.025 | 0.032 | 0.32 | 35.8 |
| stage/render/cleanup | 0.025 | 0.035 | 0.28 | 35.1 |
| encode/camera_driver_span_end/sum | 0.024 | 0.041 | 0.09 | 33.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 15.7 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 9.3 |
| controller/avoid/ms | 0.006 | 0.007 | 0.17 | 8.6 |
| script/ui/ms | 0.005 | 0.007 | 0.06 | 7.2 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 6.7 |
| anim/apply/ms | 0.004 | 0.005 | 0.11 | 6.3 |
| controller/write/ms | 0.002 | 0.002 | 0.05 | 2.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 1.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.9 |
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
| encode//max | ms | 1.61 | 2.86 | 1.66 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.19 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.09 | 0.02 |
| encode/finish_critical | ms | 1.61 | 2.86 | 1.66 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.20 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.51 | 13.53 | 9.72 |
| gpu/timed/ms | ms | 9.51 | 12.32 | 9.72 |
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
| phases/opaque3d/batch_sets | count | 20 | 20 | 20 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 15 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 22 | 19 |
| phases/shadow/bins | count | 1893 | 2200 | 1893 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5194 | 5520 | 5194 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 19 | 22 | 19 |
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
| preprocess/bind_groups_reused | count | 161 | 161 | 161 |
| process/cpu_usage | % | 25.84 | 40.36 | 24.51 |
| process/mem_usage | GiB | 4.63 | 4.72 | 4.72 |
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
| system/cpu_usage | % | 32.71 | 43.73 | 35.67 |
| system/mem_usage | % | 66.81 | 66.95 | 66.91 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2544 | 2544 | 2544 |
| textures/images | count | 2009 | 2009 | 2009 |
| views/all/batch_sets | count | 69 | 72 | 69 |
| views/all/bins | count | 5063 | 5370 | 5063 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 14.8 k | 14.5 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 61 | 61 | 61 |
| views/all/work_items | count | 14.7 k | 15.0 k | 14.7 k |
| views/cascade/batch_sets | count | 19 | 22 | 19 |
| views/cascade/bins | count | 1893 | 2200 | 1893 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5194 | 5520 | 5194 |
| views/cascade/pipelines | count | 19 | 22 | 19 |
| views/cascade/work_items | count | 5194 | 5520 | 5194 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 50 | 50 | 50 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 46 | 46 | 46 |
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
| anim/events/count | count | 0.09 | 3 | 127 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.47 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 31.25 | 33 | 44.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1425 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1425 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1425 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.63 | 1 | 900 |
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

## Hitches (> 26.2 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2521.5 | main/busy/ms 2491, stage/main/post_update 2384, stage/main/update 101, surface/advance_cpu_ms 71.44, stage/main/pre_update 2.47, stage/main/last 0.99, surface/cell_upload_ms 0.68, stage/main/first 0.45, stream/drive/ms 0.43, stage/main/fixed_main_loop 0.41, weather/eval/ms 0.17, bigspace/total/ms 0.10 |
| warm-up | 3.25 | 732.6 | main/busy/ms 2630, stage/main/post_update 2409, render/thread/ms 704, render/busy/ms 547, stage/render/prepare_assets 438, stage/main/update 210, stage/render/prepare_views 155, render/acquire/ms 155, stage/render/create_views 91.92, surface/advance_cpu_ms 71.44, stage/render/render 9.41, encode//sum 8.79 |
| warm-up | 3.33 | 76.6 | render/thread/ms 764, render/busy/ms 604, stage/render/prepare_assets 460, main/busy/ms 159, stage/render/prepare_views 156, render/acquire/ms 156, pipelines/compile_ms 150, pipelines/front_ms 144, stage/main/update 113, stage/render/create_views 92.31, encode//sum 45.40, stage/render/render 37.63 |
| warm-up | 3.36 | 31.8 | pipelines/compile_ms 155, pipelines/front_ms 147, render/thread/ms 72.63, render/busy/ms 68.80, main/busy/ms 38.25, encode//sum 37.23, stage/render/render 34.17, render/graph/ms 29.80, terrain/update/ms 24.90, stage/main/post_update 23.67, stage/render/prepare_assets 23.02, pipelines/wait_ms 17.05 |
| warm-up | 3.52 | 154.7 | pipelines/wait_ms 4637, pipelines/compile_ms 1873, pipelines/front_ms 1771, render/thread/ms 149, render/busy/ms 146, pipelines/driver_ms 101, main/busy/ms 64.19, stage/render/prepare_resources 36.19, stage/render/prepare_flush 33.70, stage/main/post_update 30.88, encode//sum 29.12, stage/render/prepare_write_phase_buffers 23.99 |
| warm-up | 3.60 | 87.6 | pipelines/wait_ms 6596, pipelines/compile_ms 2839, pipelines/front_ms 2693, render/thread/ms 218, render/busy/ms 209, main/busy/ms 155, pipelines/driver_ms 146, stage/main/update 87.45, stage/render/prepare_assets 52.33, stage/render/prepare_resources 41.77, stage/render/prepare_flush 35.25, encode//sum 32.48 |
| warm-up | 3.73 | 125.3 | pipelines/wait_ms 2180, pipelines/compile_ms 2158, pipelines/front_ms 2073, render/thread/ms 173, render/busy/ms 165, main/busy/ms 129, stage/render/prepare_assets 91.93, stage/main/update 86.91, pipelines/driver_ms 84.94, stage/render/prepare_resources 32.85, stage/render/prepare_batch_phases 19.43, stage/main/post_update 18.85 |
| warm-up | 3.77 | 42.3 | pipelines/compile_ms 1796, pipelines/front_ms 1707, pipelines/wait_ms 369, render/thread/ms 132, render/busy/ms 130, stage/render/prepare_assets 92.80, pipelines/driver_ms 89.13, main/busy/ms 43.85, stage/render/prepare_resources 26.28, stage/main/post_update 15.71, terrain/update/ms 12.55, stage/main/update 11.33 |
| warm-up | 3.90 | 43.9 | pipelines/compile_ms 207, pipelines/front_ms 190, main/busy/ms 59.97, render/thread/ms 59.26, render/busy/ms 50.00, stage/main/update 49.61, stream/drive/ms 40.99, stage/render/prepare_assets 32.44, stage/render/render 19.38, pipelines/driver_ms 16.71, render/graph/ms 9.85, encode//sum 9.02 |
| warm-up | 3.95 | 46.3 | pipelines/compile_ms 952, pipelines/front_ms 901, render/thread/ms 74.00, render/busy/ms 64.83, main/busy/ms 54.88, pipelines/driver_ms 50.43, stage/render/prepare_assets 48.72, stage/main/update 45.30, stream/drive/ms 41.23, stage/render/render 19.55, render/graph/ms 10.16, render/present/ms 8.75 |
| warm-up | 3.97 | 28.5 | pipelines/compile_ms 1008, pipelines/front_ms 966, render/thread/ms 59.21, render/busy/ms 57.39, pipelines/driver_ms 42.24, main/busy/ms 38.26, stage/render/prepare_assets 27.99, stage/main/update 26.25, stream/drive/ms 20.45, stage/render/render 13.45, render/graph/ms 11.36, stage/render/prepare_resources 9.70 |
| warm-up | 4.03 | 58.7 | pipelines/compile_ms 754, pipelines/front_ms 727, render/thread/ms 72.51, render/busy/ms 70.06, main/busy/ms 46.68, stage/main/update 27.67, pipelines/driver_ms 27.47, stage/render/prepare_assets 26.68, stage/render/render 26.64, render/graph/ms 23.75, stream/drive/ms 21.53, stage/main/post_update 13.91 |
| warm-up | 4.06 | 32.3 | pipelines/compile_ms 522, pipelines/front_ms 502, main/busy/ms 76.45, render/thread/ms 71.64, render/busy/ms 68.89, stage/main/update 52.59, stage/render/prepare_assets 37.58, stage/render/render 26.05, render/graph/ms 22.63, stream/drive/ms 20.23, pipelines/driver_ms 19.97, stage/main/post_update 15.43 |
| warm-up | 4.22 | 155.0 | pipelines/compile_ms 1425, pipelines/front_ms 1385, render/thread/ms 113, render/busy/ms 108, main/busy/ms 83.30, stage/render/prepare_assets 67.28, stage/main/update 56.06, pipelines/driver_ms 40.09, stage/render/render 37.57, render/graph/ms 31.40, stage/render/camera_driver 21.74, stream/drive/ms 21.20 |
| warm-up | 4.33 | 106.4 | pipelines/compile_ms 2045, pipelines/front_ms 1986, render/thread/ms 172, render/busy/ms 165, main/busy/ms 126, stage/main/update 88.77, stage/render/prepare_assets 84.67, stage/render/render 75.43, render/graph/ms 68.37, pipelines/driver_ms 59.59, stream/drive/ms 57.36, stage/render/camera_driver 47.27 |
| warm-up | 4.44 | 116.3 | pipelines/compile_ms 1899, pipelines/front_ms 1836, pipelines/wait_ms 208, render/thread/ms 146, render/busy/ms 143, main/busy/ms 129, stage/main/update 88.06, stage/render/prepare_assets 73.15, pipelines/driver_ms 63.70, stage/render/render 57.61, stream/drive/ms 56.54, render/graph/ms 53.95 |
| warm-up | 4.54 | 100.7 | pipelines/compile_ms 2911, pipelines/front_ms 2824, pipelines/wait_ms 541, render/thread/ms 137, render/busy/ms 136, pipelines/driver_ms 86.35, stage/render/prepare_assets 85.41, main/busy/ms 60.44, stage/main/post_update 25.50, stage/main/update 23.87, encode//sum 16.73, stage/render/render 16.64 |
| warm-up | 4.59 | 51.1 | pipelines/compile_ms 2532, pipelines/front_ms 2451, pipelines/wait_ms 757, render/thread/ms 123, render/busy/ms 121, pipelines/driver_ms 80.45, stage/render/prepare_assets 68.29, main/busy/ms 57.29, stage/main/update 30.96, stage/render/prepare_resources 19.99, stage/main/post_update 18.99, stage/render/render 11.66 |
| warm-up | 4.64 | 48.6 | pipelines/compile_ms 1308, pipelines/front_ms 1248, pipelines/wait_ms 405, render/thread/ms 75.58, render/busy/ms 74.02, pipelines/driver_ms 59.75, main/busy/ms 44.13, stage/render/prepare_assets 42.18, stage/main/update 28.39, stage/render/render 11.31, encode//sum 10.44, stage/main/post_update 9.47 |
| warm-up | 4.68 | 37.1 | pipelines/compile_ms 681, pipelines/front_ms 650, render/thread/ms 57.22, render/busy/ms 54.86, main/busy/ms 36.41, stage/render/prepare_assets 34.83, pipelines/driver_ms 30.38, pipelines/wait_ms 26.45, stage/main/update 18.70, stage/render/render 10.61, encode//sum 10.35, stage/main/post_update 9.31 |
| warm-up | 4.71 | 30.2 | pipelines/compile_ms 632, pipelines/front_ms 613, pipelines/wait_ms 185, render/thread/ms 56.84, render/busy/ms 53.99, main/busy/ms 33.40, stage/render/prepare_assets 21.26, pipelines/driver_ms 19.54, stage/main/post_update 16.53, stage/render/render 14.68, encode//sum 11.85, render/graph/ms 11.37 |
| warm-up | 4.78 | 45.3 | pipelines/compile_ms 1301, pipelines/front_ms 1261, pipelines/wait_ms 365, main/busy/ms 79.46, stage/main/update 62.66, stream/drive/ms 56.33, render/thread/ms 53.64, render/busy/ms 50.33, pipelines/driver_ms 40.40, stage/render/render 29.63, render/graph/ms 26.01, stage/render/prepare_assets 12.98 |
| warm-up | 4.83 | 50.9 | pipelines/compile_ms 1091, pipelines/front_ms 1056, main/busy/ms 115, stage/main/update 101, stream/drive/ms 91.99, render/thread/ms 52.98, render/busy/ms 49.69, pipelines/wait_ms 39.18, pipelines/driver_ms 34.35, stage/render/render 27.36, render/graph/ms 23.79, stage/render/prepare_assets 15.66 |
| warm-up | 4.89 | 58.9 | pipelines/compile_ms 670, pipelines/front_ms 649, main/busy/ms 69.98, stage/main/update 49.73, stream/drive/ms 40.86, render/thread/ms 36.42, render/busy/ms 34.52, pipelines/driver_ms 20.73, stage/render/render 13.32, stage/render/prepare_assets 13.12, stage/main/post_update 11.73, encode//sum 11.65 |
| warm-up | 4.95 | 61.3 | pipelines/compile_ms 497, pipelines/front_ms 476, main/busy/ms 92.48, render/thread/ms 62.21, render/busy/ms 56.60, stage/main/update 56.34, stage/render/render 27.51, pipelines/driver_ms 21.81, render/graph/ms 19.40, pipelines/wait_ms 18.13, stage/render/prepare_assets 16.46, encode//sum 13.69 |
| warm-up | 5.04 | 91.5 | pipelines/compile_ms 1708, pipelines/front_ms 1649, main/busy/ms 96.00, render/thread/ms 87.04, render/busy/ms 81.54, stage/main/update 63.98, pipelines/driver_ms 58.19, stage/render/prepare_assets 38.17, pipelines/wait_ms 32.58, stage/render/render 26.58, render/graph/ms 18.59, encode//sum 15.78 |
| warm-up | 5.07 | 29.9 | pipelines/compile_ms 1860, pipelines/front_ms 1806, render/thread/ms 68.55, render/busy/ms 66.76, pipelines/driver_ms 54.00, main/busy/ms 40.20, stage/render/prepare_assets 32.34, stage/main/update 21.88, pipelines/wait_ms 17.83, encode//sum 15.13, stage/render/render 13.93, render/graph/ms 11.94 |
| warm-up | 5.13 | 38.3 | pipelines/compile_ms 437, pipelines/front_ms 422, render/thread/ms 52.16, render/busy/ms 50.51, main/busy/ms 39.94, encode//sum 23.05, stage/main/update 21.13, stage/render/render 18.50, render/graph/ms 16.50, pipelines/driver_ms 14.90, stage/render/graph_submit 13.68, stage/main/post_update 13.07 |
| warm-up | 5.16 | 32.7 | pipelines/compile_ms 643, pipelines/front_ms 625, render/thread/ms 58.62, render/busy/ms 56.86, main/busy/ms 45.96, stage/main/update 27.12, encode//sum 23.00, stage/render/render 20.93, render/graph/ms 18.83, pipelines/driver_ms 18.25, stream/drive/ms 15.97, stage/render/graph_submit 15.70 |
| warm-up | 5.20 | 33.7 | pipelines/compile_ms 529, pipelines/front_ms 516, render/thread/ms 55.57, render/busy/ms 54.04, main/busy/ms 42.74, stage/main/update 22.84, encode//sum 16.79, stream/drive/ms 16.17, stage/render/render 14.94, pipelines/driver_ms 13.14, render/graph/ms 13.07, stage/render/graph_submit 10.43 |
| warm-up | 5.23 | 33.7 | pipelines/compile_ms 537, pipelines/front_ms 521, render/thread/ms 59.52, render/busy/ms 58.13, main/busy/ms 44.27, pipelines/wait_ms 27.67, stage/render/prepare_resources 19.59, stage/main/update 18.37, pipelines/driver_ms 16.17, encode//sum 14.67, stage/main/post_update 14.33, stage/render/render 12.59 |
| warm-up | 5.27 | 35.0 | pipelines/compile_ms 1006, pipelines/front_ms 975, render/thread/ms 59.87, render/busy/ms 58.09, main/busy/ms 44.79, pipelines/driver_ms 31.16, pipelines/wait_ms 27.89, stage/main/update 21.68, stage/main/post_update 15.85, encode//sum 15.26, stage/render/render 14.29, stage/render/prepare_resources 13.62 |
| warm-up | 5.29 | 26.4 | pipelines/compile_ms 981, pipelines/front_ms 951, render/thread/ms 49.56, main/busy/ms 48.42, render/busy/ms 47.43, pipelines/driver_ms 30.65, stage/main/update 28.53, stage/render/prepare_assets 16.93, stage/render/render 16.93, encode//sum 15.33, render/graph/ms 14.44, stream/drive/ms 11.79 |
| warm-up | 5.35 | 52.9 | pipelines/compile_ms 1050, pipelines/front_ms 1021, render/thread/ms 56.42, render/busy/ms 54.43, main/busy/ms 43.18, pipelines/wait_ms 42.27, pipelines/driver_ms 28.76, stage/main/update 24.95, stage/render/prepare_assets 23.29, stage/render/render 18.88, render/graph/ms 16.65, encode//sum 15.88 |
| warm-up | 5.38 | 29.7 | pipelines/compile_ms 1343, pipelines/front_ms 1307, render/thread/ms 62.10, render/busy/ms 59.94, pipelines/wait_ms 42.12, main/busy/ms 41.75, pipelines/driver_ms 36.56, stage/render/prepare_assets 21.32, stage/render/render 19.51, encode//sum 18.70, render/graph/ms 16.81, stage/main/update 15.32 |
| warm-up | 5.41 | 38.3 | pipelines/compile_ms 786, pipelines/front_ms 762, render/thread/ms 52.75, render/busy/ms 50.89, main/busy/ms 42.63, pipelines/driver_ms 23.35, stage/render/prepare_assets 20.52, encode//sum 17.14, stage/main/update 16.94, stage/render/render 14.93, render/graph/ms 12.45, stage/main/post_update 11.38 |
| warm-up | 5.44 | 27.9 | pipelines/compile_ms 282, pipelines/front_ms 273, render/thread/ms 48.85, render/busy/ms 47.18, main/busy/ms 34.00, stage/render/prepare_assets 27.50, stage/main/update 20.11, encode//sum 14.20, stage/render/render 11.56, stream/drive/ms 10.80, render/graph/ms 9.48, pipelines/driver_ms 8.26 |
| warm-up | 5.65 | 48.0 | pipelines/compile_ms 141, pipelines/front_ms 138, render/thread/ms 64.69, render/busy/ms 62.97, stage/render/prepare_assets 34.17, main/busy/ms 22.82, encode//sum 18.66, stage/render/render 14.16, render/graph/ms 11.75, stage/main/update 10.07, stage/render/graph_submit 8.65, stage/main/post_update 7.70 |
| warm-up | 5.70 | 43.5 | render/thread/ms 43.67, render/busy/ms 42.62, main/busy/ms 25.97, stage/render/prepare_assets 24.11, encode//sum 14.49, stage/main/update 11.81, stage/render/render 9.68, stage/main/post_update 9.32, render/graph/ms 8.24, stage/render/graph_submit 6.15, stream/drive/ms 3.81, encode/submit 3.37 |
| warm-up | 5.78 | 30.0 | render/thread/ms 46.78, render/busy/ms 45.62, main/busy/ms 32.55, stage/render/prepare_assets 27.96, stage/main/update 14.41, encode//sum 12.82, stage/main/post_update 12.42, stage/render/render 8.74, render/graph/ms 7.23, stage/render/graph_submit 5.12, stream/drive/ms 4.08, encode/finish_wall 2.69 |
| warm-up | 5.81 | 31.1 | render/thread/ms 53.24, render/busy/ms 51.65, main/busy/ms 36.22, stage/render/prepare_assets 25.67, encode//sum 18.54, stage/main/update 16.57, stage/main/post_update 13.69, stage/render/render 13.38, render/graph/ms 11.13, stage/render/graph_submit 8.22, stream/drive/ms 5.98, encode/submit 4.18 |
| warm-up | 5.84 | 31.6 | render/thread/ms 49.90, render/busy/ms 48.04, main/busy/ms 32.63, encode//sum 23.01, stage/render/prepare_assets 19.09, stage/render/render 16.69, stage/main/post_update 14.29, render/graph/ms 13.86, stage/main/update 12.61, stage/render/graph_submit 10.12, stream/drive/ms 5.85, encode/finish_wall 5.08 |
| warm-up | 5.88 | 35.1 | render/thread/ms 45.41, render/busy/ms 44.00, main/busy/ms 29.49, stage/render/prepare_assets 23.35, encode//sum 17.36, stage/render/render 12.52, stage/main/post_update 12.06, stage/main/update 11.25, render/graph/ms 10.50, stage/render/graph_submit 7.71, encode/submit 4.01, stream/drive/ms 3.91 |
| warm-up | 5.93 | 29.3 | main/busy/ms 42.12, render/thread/ms 32.66, render/busy/ms 31.15, stage/main/update 25.42, encode//sum 17.07, stream/drive/ms 14.06, stage/render/render 12.33, stage/render/prepare_assets 10.88, render/graph/ms 10.32, stage/main/post_update 10.29, stage/render/graph_submit 8.13, pipelines/compile_ms 7.98 |
| warm-up | 6.08 | 29.8 | main/busy/ms 32.46, render/thread/ms 24.87, render/busy/ms 23.67, stage/main/update 21.14, encode//sum 15.14, stream/drive/ms 14.84, stage/render/render 10.82, render/graph/ms 9.24, stage/main/post_update 7.51, stage/render/prepare_assets 7.25, stage/render/graph_submit 6.77, encode/finish_wall 3.63 |
| warm-up | 6.39 | 30.5 | main/busy/ms 42.18, render/thread/ms 42.01, render/busy/ms 39.99, encode//sum 29.32, stage/render/render 21.13, stage/main/update 18.66, render/graph/ms 18.36, stage/main/post_update 16.85, stage/render/graph_submit 14.71, stage/render/prepare_assets 11.66, stream/drive/ms 10.93, encode/submit 7.88 |
| warm-up | 6.43 | 39.1 | render/thread/ms 51.38, render/busy/ms 49.88, main/busy/ms 43.43, stage/render/prepare_assets 27.27, encode//sum 23.29, stage/main/update 19.23, stage/main/post_update 17.27, stage/render/render 14.68, render/graph/ms 12.63, stream/drive/ms 10.93, stage/render/graph_submit 9.85, encode/finish_wall 6.03 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_off\balmora@2560x1440_off=cascadeencoders_cascadehold_meshslabs-3 --bench-baseline none --off cascadeencoders,cascadehold,meshslabs
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.1 W · 73 °C (max 75) · VRAM 5672 MiB · throttled 100% of samples
