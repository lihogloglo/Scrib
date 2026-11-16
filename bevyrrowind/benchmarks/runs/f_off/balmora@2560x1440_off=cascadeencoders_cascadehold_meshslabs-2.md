# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs,cascadeencoders,cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1410 frames over 15.0 s · **median 10.58 ms (94 fps)** · mean 10.64 · p95 11.89 · p99 12.72 (1% low 79 fps) · worst 14.38 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 280 frames over 8.7 s, worst 2614.2 ms
- hitches (> 26.5 ms): **0** measured, 52 in warm-up
- timed GPU passes: 8.85 ms of a 10.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.58 ms, limit 11 ms
- pass worst frame 14.38 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.858 | 2.905 | 3.256 | 0.98 | 0.068 |
| dlss_super_resolution | 2.026 | 2.060 | 2.093 | 0.98 | 0.185 |
| early prepass | 0.869 | 0.884 | 1.148 | 0.98 | 0.088 |
| atmosphere_luts | 0.376 | 0.383 | 0.708 | 0.98 | 0.004 |
| bloom | 0.314 | 0.319 | 0.334 | 0.98 | 0.066 |
| main_transmissive_pass_3d | 0.280 | 0.285 | 0.299 | 0.98 | 0.009 |
| water_velocity | 0.242 | 0.246 | 0.268 | 0.98 | 0.002 |
| water_sspr | 0.211 | 0.214 | 0.217 | 0.98 | 0.002 |
| ssao | 0.197 | 0.200 | 0.209 | 0.98 | 0.008 |
| uniform_allocation | 0.149 | 0.152 | 0.156 | 0.98 | 0.031 |
| clouds_trace | 0.149 | 0.151 | 0.183 | 0.98 | 0.001 |
| atmos_froxel_inject | 0.148 | 0.150 | 0.153 | 0.98 | 0.001 |
| water_fft | 0.133 | 0.135 | 0.138 | 0.98 | 0.003 |
| upscaling | 0.116 | 0.118 | 0.124 | 0.98 | 0.004 |
| clouds_composite | 0.112 | 0.114 | 0.116 | 0.98 | 0.002 |
| bin_unpacking | 0.108 | 0.110 | 0.114 | 0.98 | 0.023 |
| tonemapping | 0.086 | 0.087 | 0.092 | 0.98 | 0.004 |
| clustering | 0.065 | 0.066 | 0.078 | 0.98 | 0.022 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.421 | 0.419 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.041 | 0.042 | 0.041 | 0.98 | 0.008 |
| clouds_shadow | 0.030 | 0.031 | 0.035 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.223 | 0.234 | 0.12 | 0.005 |
| main_transparent_pass_3d | 0.026 | 0.027 | 0.034 | 0.98 | 0.075 |
| water_snapshot | 0.024 | 0.024 | 0.027 | 0.98 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.030 | 0.98 | 0.008 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 0.98 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.98 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.009 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.015 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.702 | 10.942 | 13.29 | 13680.1 |
| encode//sum | 8.843 | 11.956 | 15.43 | 12469.1 |
| render/busy/ms | 8.184 | 9.623 | 11.18 | 11538.8 |
| main/busy/ms | 7.632 | 9.053 | 11.00 | 10760.5 |
| stage/render/render | 5.780 | 7.122 | 10.20 | 8149.6 |
| render/graph/ms | 5.048 | 6.253 | 7.78 | 7118.2 |
| stage/main/post_update | 4.642 | 5.975 | 8.38 | 6545.6 |
| stage/render/graph_submit | 3.996 | 5.175 | 6.68 | 5633.7 |
| encode/finish_wall | 2.107 | 2.646 | 3.27 | 2970.8 |
| encode/submit | 1.946 | 2.722 | 6.18 | 2744.0 |
| stage/main/update | 1.866 | 2.223 | 3.91 | 2631.4 |
| stage/render/prepare_views | 1.069 | 2.155 | 3.37 | 1507.0 |
| render/acquire/ms | 1.024 | 2.107 | 3.32 | 1443.2 |
| stage/render/camera_driver | 0.993 | 1.173 | 1.58 | 1399.5 |
| stage/render/prepare_write_phase_buffers | 0.743 | 0.913 | 1.27 | 1048.0 |
| render/present/ms | 0.495 | 0.612 | 2.71 | 698.1 |
| stage/render/prepare_assets | 0.457 | 0.583 | 1.59 | 644.6 |
| bigspace/total/ms | 0.418 | 1.044 | 3.16 | 589.9 |
| stage/render/prepare_resources | 0.378 | 0.476 | 0.74 | 533.1 |
| stage/main/pre_update | 0.325 | 0.433 | 0.68 | 457.8 |
| stage/main/first | 0.301 | 0.366 | 1.54 | 424.6 |
| terrain/update/ms | 0.236 | 0.299 | 0.56 | 332.2 |
| stage/render/prepare_bind_groups | 0.208 | 0.303 | 0.65 | 292.9 |
| terrain/select/ms | 0.205 | 0.260 | 0.53 | 289.3 |
| stream/drive/ms | 0.200 | 0.291 | 0.58 | 282.4 |
| stage/main/fixed_main_loop | 0.198 | 0.364 | 0.90 | 279.0 |
| stage/render/prepare_flush | 0.192 | 0.386 | 0.84 | 270.4 |
| stage/render/prepare_meshes | 0.186 | 0.240 | 0.43 | 262.4 |
| controller/feed/ms | 0.186 | 0.243 | 0.64 | 262.3 |
| stage/render/extract_commands | 0.176 | 0.210 | 0.43 | 247.5 |
| stream/tick/ms | 0.171 | 0.256 | 0.53 | 241.3 |
| far/select/ms | 0.167 | 0.253 | 0.56 | 235.2 |
| stage/render/prepare_batch_phases | 0.167 | 0.229 | 0.42 | 234.8 |
| bigspace/high_precision/ms | 0.163 | 0.544 | 2.96 | 229.7 |
| stage/render/create_views | 0.149 | 0.196 | 0.46 | 210.5 |
| stage/main/last | 0.115 | 0.155 | 0.67 | 162.0 |
| bigspace/low_precision/ms | 0.110 | 0.222 | 2.61 | 154.9 |
| stage/render/queue | 0.109 | 0.146 | 0.42 | 153.3 |
| anim/evaluate/ms | 0.105 | 0.239 | 2.20 | 147.7 |
| controller/move/ms | 0.104 | 0.171 | 0.33 | 146.1 |
| stage/render/specialize | 0.097 | 0.131 | 0.44 | 136.1 |
| anim/evaluate_cpu/ms | 0.059 | 0.077 | 0.34 | 83.9 |
| stage/render/prepare_collect_phase_buffers | 0.051 | 0.083 | 0.35 | 72.4 |
| encode/resolve_diagnostics/sum | 0.044 | 0.080 | 0.14 | 61.8 |
| stage/render/phase_sort | 0.044 | 0.103 | 0.23 | 61.6 |
| physics/step/ms | 0.031 | 0.059 | 0.27 | 44.1 |
| weather/map/ms | 0.027 | 0.035 | 0.23 | 37.4 |
| stage/render/cleanup | 0.024 | 0.034 | 0.31 | 34.4 |
| encode/camera_driver_span_begin/sum | 0.021 | 0.037 | 0.29 | 29.7 |
| encode/camera_driver_span_end/sum | 0.017 | 0.033 | 0.15 | 24.0 |
| weather/eval/ms | 0.012 | 0.016 | 0.25 | 16.9 |
| script/update/ms | 0.008 | 0.011 | 0.06 | 10.6 |
| controller/avoid/ms | 0.006 | 0.008 | 0.31 | 8.5 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.3 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 7.0 |
| anim/apply/ms | 0.005 | 0.006 | 0.20 | 6.4 |
| controller/write/ms | 0.002 | 0.002 | 0.14 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.14 | 0.7 |
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
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4212 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.78 | 2.96 | 1.69 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.29 | 0.04 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.15 | 0.02 |
| encode/finish_critical | ms | 1.78 | 2.96 | 1.69 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.14 | 0.07 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.61 | 11.18 | 9.62 |
| gpu/timed/ms | ms | 9.60 | 11.13 | 9.62 |
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
| preprocess/bind_groups_reused | count | 165 | 165 | 165 |
| process/cpu_usage | % | 27.88 | 46.15 | 24.04 |
| process/mem_usage | GiB | 4.67 | 4.71 | 4.71 |
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
| system/cpu_usage | % | 35.82 | 44.28 | 33.50 |
| system/mem_usage | % | 67.03 | 67.22 | 67.22 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2544 | 2544 | 2544 |
| textures/images | count | 2009 | 2009 | 2009 |
| views/all/batch_sets | count | 71 | 74 | 71 |
| views/all/bins | count | 5065 | 5372 | 5065 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 14.8 k | 14.5 k |
| views/all/pipelines | count | 67 | 70 | 67 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
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
| anim/events/count | count | 0.10 | 2 | 135 |
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
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 31.25 | 33 | 44.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1410 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1410 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1410 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 901 |
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

## Hitches (> 26.5 ms; slowest 52, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.61 | 2614.2 | main/busy/ms 2585, stage/main/post_update 2485, stage/main/update 93.52, surface/advance_cpu_ms 65.58, stage/main/pre_update 2.56, stage/main/last 1.04, surface/cell_upload_ms 0.85, stage/main/first 0.60, stage/main/fixed_main_loop 0.46, stream/drive/ms 0.39, weather/map/ms 0.12, bigspace/total/ms 0.11 |
| warm-up | 3.24 | 626.5 | main/busy/ms 2766, stage/main/post_update 2511, render/thread/ms 594, render/busy/ms 495, stage/render/prepare_assets 469, stage/main/update 243, stage/render/prepare_views 97.49, render/acquire/ms 97.31, surface/advance_cpu_ms 65.58, encode//sum 10.86, stage/render/render 9.84, stage/render/create_views 8.27 |
| warm-up | 3.30 | 59.7 | render/thread/ms 637, render/busy/ms 536, stage/render/prepare_assets 493, main/busy/ms 199, stage/main/update 153, stage/render/prepare_views 97.81, render/acquire/ms 97.52, pipelines/compile_ms 60.49, pipelines/front_ms 57.54, stage/main/post_update 35.42, terrain/update/ms 20.42, stage/render/render 19.54 |
| warm-up | 3.33 | 32.1 | pipelines/compile_ms 204, pipelines/front_ms 185, render/thread/ms 73.82, render/busy/ms 68.84, main/busy/ms 66.94, pipelines/wait_ms 56.01, stage/main/post_update 51.37, stage/render/render 31.76, render/graph/ms 26.17, terrain/update/ms 25.75, stage/render/prepare_assets 25.07, pipelines/driver_ms 19.42 |
| warm-up | 3.45 | 117.2 | pipelines/wait_ms 1887, pipelines/compile_ms 773, pipelines/front_ms 700, render/thread/ms 128, render/busy/ms 123, pipelines/driver_ms 72.84, main/busy/ms 63.91, stage/main/post_update 51.99, stage/render/prepare_resources 29.45, stage/render/render 25.38, stage/render/prepare_write_phase_buffers 24.03, pipelines/lock_ms 23.53 |
| warm-up | 3.54 | 92.6 | pipelines/wait_ms 5402, pipelines/compile_ms 2379, pipelines/front_ms 2249, render/thread/ms 188, render/busy/ms 185, pipelines/driver_ms 130, stage/render/prepare_write_phase_buffers 55.24, main/busy/ms 51.35, stage/render/prepare_resources 46.53, stage/render/prepare_assets 36.32, stage/main/post_update 34.09, stage/render/prepare_bind_groups 23.94 |
| warm-up | 3.58 | 41.5 | pipelines/wait_ms 3595, pipelines/compile_ms 1983, pipelines/front_ms 1901, render/thread/ms 130, render/busy/ms 127, main/busy/ms 102, pipelines/driver_ms 82.74, stage/main/update 60.89, stage/render/prepare_assets 46.33, stage/render/prepare_write_phase_buffers 32.17, stage/main/post_update 28.99, stage/render/prepare_resources 21.89 |
| warm-up | 3.68 | 100.9 | pipelines/compile_ms 1492, pipelines/front_ms 1425, pipelines/wait_ms 381, render/thread/ms 109, render/busy/ms 100, main/busy/ms 83.08, pipelines/driver_ms 66.42, stage/render/prepare_assets 63.59, stage/main/update 59.90, stage/render/render 35.60, render/graph/ms 26.18, stage/render/graph_submit 21.68 |
| warm-up | 3.77 | 43.5 | pipelines/compile_ms 723, pipelines/front_ms 695, pipelines/wait_ms 75.70, render/thread/ms 65.78, render/busy/ms 63.50, main/busy/ms 39.22, pipelines/driver_ms 28.47, stage/main/post_update 26.89, encode//sum 19.64, stage/render/render 15.00, stage/render/prepare_assets 13.56, render/graph/ms 12.29 |
| warm-up | 3.84 | 47.6 | pipelines/compile_ms 271, pipelines/front_ms 264, render/thread/ms 63.34, render/busy/ms 61.14, stage/render/prepare_assets 34.70, main/busy/ms 34.15, stage/render/render 13.37, stage/main/update 10.91, render/graph/ms 10.33, stage/main/fixed_main_loop 10.26, encode//sum 9.29, stage/render/graph_submit 7.78 |
| warm-up | 3.95 | 84.7 | pipelines/compile_ms 844, pipelines/front_ms 801, render/thread/ms 70.76, render/busy/ms 67.95, main/busy/ms 65.60, stage/main/update 48.46, pipelines/driver_ms 42.16, stream/drive/ms 40.79, stage/render/prepare_assets 40.54, stage/render/render 15.59, render/graph/ms 13.01, stage/main/post_update 11.80 |
| warm-up | 3.99 | 42.6 | pipelines/compile_ms 555, pipelines/front_ms 538, render/thread/ms 84.10, render/busy/ms 72.89, main/busy/ms 50.56, stage/render/prepare_assets 41.04, stage/main/update 25.79, stream/drive/ms 18.70, pipelines/driver_ms 17.21, stage/render/render 16.62, stage/main/post_update 16.58, render/graph/ms 10.60 |
| warm-up | 4.05 | 59.4 | pipelines/compile_ms 1031, pipelines/front_ms 993, render/thread/ms 87.20, render/busy/ms 69.08, main/busy/ms 52.80, pipelines/driver_ms 37.83, stage/render/prepare_assets 29.35, stage/render/render 26.27, stage/main/update 23.80, stream/drive/ms 19.71, stage/render/prepare_flush 14.30, stage/main/post_update 14.03 |
| warm-up | 4.10 | 53.9 | pipelines/compile_ms 798, pipelines/front_ms 767, render/thread/ms 96.57, render/busy/ms 87.54, main/busy/ms 57.30, stage/render/prepare_assets 53.96, pipelines/driver_ms 31.33, stage/main/update 26.25, pipelines/wait_ms 22.89, stage/render/render 22.55, pipelines/lock_ms 22.32, stream/drive/ms 20.30 |
| warm-up | 4.21 | 102.0 | pipelines/compile_ms 1067, pipelines/front_ms 1029, render/thread/ms 122, render/busy/ms 120, main/busy/ms 115, stage/main/update 64.23, stage/render/prepare_assets 60.62, stage/main/post_update 41.42, pipelines/driver_ms 38.12, stage/render/render 35.38, render/graph/ms 32.30, stage/render/camera_driver 21.74 |
| warm-up | 4.27 | 61.0 | pipelines/compile_ms 1367, pipelines/front_ms 1324, main/busy/ms 201, stage/main/update 122, render/thread/ms 119, render/busy/ms 116, stage/main/post_update 69.99, stream/drive/ms 68.19, stage/render/prepare_assets 57.26, pipelines/driver_ms 43.52, stage/render/render 36.58, render/graph/ms 33.64 |
| warm-up | 4.47 | 207.6 | pipelines/compile_ms 2758, pipelines/front_ms 2690, main/busy/ms 144, render/thread/ms 130, render/busy/ms 127, stage/main/update 86.49, stream/drive/ms 68.19, pipelines/driver_ms 67.46, stage/render/prepare_assets 66.86, stage/main/post_update 47.45, pipelines/wait_ms 33.69, stage/render/prepare_write_phase_buffers 32.56 |
| warm-up | 4.58 | 103.9 | pipelines/compile_ms 3575, pipelines/front_ms 3480, pipelines/wait_ms 183, render/thread/ms 166, render/busy/ms 164, stage/render/prepare_assets 95.80, pipelines/driver_ms 95.44, main/busy/ms 76.86, stage/main/update 48.53, stage/render/prepare_write_phase_buffers 32.54, stage/render/render 22.09, render/graph/ms 19.21 |
| warm-up | 4.69 | 108.3 | pipelines/compile_ms 2419, pipelines/front_ms 2331, pipelines/wait_ms 296, render/thread/ms 163, render/busy/ms 161, stage/render/prepare_assets 96.06, pipelines/driver_ms 88.13, main/busy/ms 80.84, stage/main/update 47.86, stage/main/fixed_main_loop 19.04, stage/render/render 18.81, stage/render/prepare_write_phase_buffers 18.56 |
| warm-up | 4.72 | 31.4 | pipelines/compile_ms 2460, pipelines/front_ms 2376, pipelines/wait_ms 544, render/thread/ms 114, render/busy/ms 112, pipelines/driver_ms 84.03, main/busy/ms 53.06, stage/render/prepare_assets 49.02, stage/main/update 21.21, stage/render/prepare_write_phase_buffers 18.47, stage/render/render 16.93, stage/main/fixed_main_loop 15.47 |
| warm-up | 4.75 | 30.8 | pipelines/compile_ms 1275, pipelines/front_ms 1240, pipelines/wait_ms 399, render/thread/ms 52.42, render/busy/ms 50.72, main/busy/ms 45.42, pipelines/driver_ms 35.41, stage/main/update 23.27, stage/render/prepare_assets 18.24, stage/render/render 16.33, stage/main/post_update 15.99, render/graph/ms 14.17 |
| warm-up | 4.78 | 32.9 | pipelines/compile_ms 325, pipelines/front_ms 313, render/thread/ms 54.34, render/busy/ms 52.21, main/busy/ms 45.58, stage/main/post_update 25.84, stage/render/render 22.55, render/graph/ms 19.41, encode//sum 13.44, stage/main/update 13.41, stage/render/graph_submit 13.30, stage/render/prepare_assets 12.31 |
| warm-up | 4.81 | 28.7 | pipelines/compile_ms 686, pipelines/front_ms 663, render/thread/ms 54.20, render/busy/ms 50.96, main/busy/ms 45.55, pipelines/wait_ms 27.57, stage/render/render 26.51, stage/main/post_update 26.40, render/graph/ms 22.38, pipelines/driver_ms 22.08, encode//sum 16.74, stage/render/graph_submit 15.68 |
| warm-up | 4.84 | 27.9 | pipelines/compile_ms 432, pipelines/front_ms 418, main/busy/ms 64.70, stage/main/update 49.20, stream/drive/ms 42.59, render/thread/ms 40.61, render/busy/ms 38.00, pipelines/wait_ms 27.09, encode//sum 16.27, stage/render/render 14.49, pipelines/driver_ms 14.09, stage/render/prepare_assets 14.06 |
| warm-up | 4.89 | 49.7 | main/busy/ms 91.79, stage/main/update 78.16, stream/drive/ms 71.76, render/thread/ms 34.00, render/busy/ms 32.41, pipelines/compile_ms 19.72, pipelines/front_ms 18.37, stage/render/prepare_assets 13.83, encode//sum 12.84, stage/render/render 10.85, render/graph/ms 8.68, stage/main/post_update 7.57 |
| warm-up | 4.96 | 71.7 | pipelines/compile_ms 837, pipelines/front_ms 817, pipelines/wait_ms 234, main/busy/ms 94.34, stage/main/update 68.51, render/thread/ms 42.65, render/busy/ms 36.62, stream/drive/ms 33.64, pipelines/driver_ms 20.92, stage/render/render 20.40, stage/main/post_update 15.58, render/graph/ms 13.36 |
| warm-up | 5.03 | 73.1 | pipelines/compile_ms 1168, pipelines/front_ms 1135, pipelines/wait_ms 258, main/busy/ms 87.77, render/thread/ms 67.88, render/busy/ms 61.70, stage/main/update 56.75, stage/render/prepare_assets 34.48, pipelines/driver_ms 33.09, pipelines/lock_ms 24.82, stage/render/render 24.39, render/graph/ms 17.47 |
| warm-up | 5.08 | 42.3 | pipelines/compile_ms 387, pipelines/front_ms 372, render/thread/ms 76.23, render/busy/ms 74.29, main/busy/ms 61.28, stage/main/update 39.65, stage/render/prepare_assets 39.59, pipelines/wait_ms 25.47, pipelines/lock_ms 24.95, stage/render/render 17.92, render/graph/ms 15.59, pipelines/driver_ms 15.43 |
| warm-up | 5.14 | 62.2 | pipelines/compile_ms 1029, pipelines/front_ms 1000, render/thread/ms 92.19, render/busy/ms 90.32, main/busy/ms 66.13, stage/main/update 35.22, stage/render/prepare_resources 30.76, pipelines/driver_ms 28.64, stage/main/post_update 24.49, stage/render/prepare_assets 21.91, stage/render/render 15.15, encode//sum 14.39 |
| warm-up | 5.17 | 27.7 | pipelines/compile_ms 1418, pipelines/front_ms 1376, render/thread/ms 84.10, render/busy/ms 82.36, main/busy/ms 67.34, pipelines/driver_ms 42.08, stage/main/update 35.11, stage/render/prepare_resources 31.71, stage/main/post_update 25.94, pipelines/wait_ms 15.95, encode//sum 15.64, pipelines/lock_ms 15.64 |
| warm-up | 5.20 | 38.1 | pipelines/compile_ms 616, pipelines/front_ms 594, render/thread/ms 59.52, render/busy/ms 57.79, main/busy/ms 48.32, stage/main/update 23.99, pipelines/driver_ms 22.26, encode//sum 17.70, stage/main/post_update 17.48, stage/render/prepare_assets 17.30, stage/render/render 14.91, render/graph/ms 12.85 |
| warm-up | 5.24 | 32.9 | pipelines/compile_ms 434, pipelines/front_ms 420, render/thread/ms 60.93, render/busy/ms 59.08, main/busy/ms 49.13, stage/main/update 25.74, stage/render/prepare_assets 24.57, encode//sum 18.27, stream/drive/ms 16.83, stage/main/post_update 16.63, stage/render/render 15.47, pipelines/driver_ms 14.69 |
| warm-up | 5.29 | 28.5 | pipelines/compile_ms 812, pipelines/front_ms 789, render/thread/ms 43.97, render/busy/ms 42.15, main/busy/ms 40.23, pipelines/wait_ms 35.29, pipelines/driver_ms 22.59, encode//sum 18.58, stage/main/post_update 18.16, stage/main/update 15.57, stage/render/render 14.52, render/graph/ms 12.44 |
| warm-up | 5.32 | 37.0 | pipelines/compile_ms 642, pipelines/front_ms 622, render/thread/ms 57.57, render/busy/ms 53.70, main/busy/ms 42.01, pipelines/driver_ms 20.15, stage/render/render 18.97, stage/main/update 18.31, encode//sum 17.34, stage/main/post_update 16.58, render/graph/ms 15.64, pipelines/wait_ms 13.51 |
| warm-up | 5.36 | 38.8 | pipelines/compile_ms 436, pipelines/front_ms 422, render/thread/ms 62.41, render/busy/ms 58.63, main/busy/ms 50.02, stage/main/update 26.73, stage/render/render 20.40, stage/render/prepare_assets 17.83, encode//sum 17.17, render/graph/ms 17.06, pipelines/wait_ms 16.61, stage/main/post_update 15.27 |
| warm-up | 5.41 | 49.0 | pipelines/compile_ms 925, pipelines/front_ms 900, render/thread/ms 68.58, render/busy/ms 66.84, main/busy/ms 47.79, stage/render/prepare_assets 26.46, pipelines/driver_ms 25.54, stage/main/update 24.80, stage/render/render 19.09, encode//sum 17.44, render/graph/ms 16.64, pipelines/wait_ms 16.34 |
| warm-up | 5.44 | 34.5 | pipelines/compile_ms 970, pipelines/front_ms 943, render/thread/ms 71.27, render/busy/ms 69.42, main/busy/ms 43.10, pipelines/driver_ms 27.53, stage/main/update 24.32, pipelines/wait_ms 23.11, stage/render/prepare_assets 19.97, encode//sum 19.56, stage/render/render 19.51, render/graph/ms 16.79 |
| warm-up | 5.50 | 59.7 | pipelines/compile_ms 808, pipelines/front_ms 785, render/thread/ms 69.15, render/busy/ms 67.54, main/busy/ms 41.54, pipelines/wait_ms 32.84, stage/render/prepare_assets 29.67, stage/main/update 25.33, pipelines/driver_ms 22.82, encode//sum 18.87, stage/render/render 15.12, render/graph/ms 12.81 |
| warm-up | 5.54 | 30.5 | pipelines/compile_ms 1055, pipelines/front_ms 1023, render/thread/ms 62.49, render/busy/ms 61.09, main/busy/ms 42.13, stage/render/prepare_assets 39.96, pipelines/driver_ms 31.34, stage/main/update 24.07, pipelines/wait_ms 23.39, encode//sum 15.79, stream/drive/ms 14.00, stage/render/render 12.64 |
| warm-up | 5.60 | 26.6 | pipelines/compile_ms 807, pipelines/front_ms 785, render/thread/ms 41.96, render/busy/ms 40.03, main/busy/ms 32.47, pipelines/driver_ms 22.06, encode//sum 19.60, stage/render/render 16.45, render/graph/ms 14.40, stage/main/post_update 14.03, stage/render/graph_submit 11.31, stage/main/update 10.93 |
| warm-up | 5.66 | 36.7 | pipelines/compile_ms 755, pipelines/front_ms 737, render/thread/ms 52.54, render/busy/ms 50.67, main/busy/ms 39.06, stage/render/prepare_assets 21.74, pipelines/driver_ms 18.22, stage/main/update 17.51, encode//sum 17.18, pipelines/wait_ms 16.13, stage/render/render 15.34, stage/main/post_update 15.21 |
| warm-up | 5.75 | 32.3 | pipelines/compile_ms 366, pipelines/front_ms 355, render/thread/ms 52.18, render/busy/ms 49.82, main/busy/ms 33.51, encode//sum 20.69, stage/render/render 18.19, render/graph/ms 15.44, stage/render/prepare_assets 14.73, stage/main/post_update 13.73, stage/main/update 13.70, pipelines/wait_ms 12.39 |
| warm-up | 5.79 | 38.8 | pipelines/compile_ms 344, pipelines/front_ms 331, render/thread/ms 59.98, render/busy/ms 57.96, main/busy/ms 36.09, stage/render/prepare_assets 27.26, encode//sum 20.36, stage/render/render 16.55, render/graph/ms 14.12, stage/main/post_update 13.38, stage/main/update 13.22, pipelines/driver_ms 13.04 |
| warm-up | 5.83 | 41.6 | pipelines/compile_ms 169, pipelines/front_ms 163, render/thread/ms 60.79, render/busy/ms 59.59, stage/render/prepare_assets 39.96, main/busy/ms 32.23, encode//sum 14.86, stage/main/update 13.82, stage/render/render 10.92, stage/main/post_update 9.38, render/graph/ms 9.32, stage/render/graph_submit 7.23 |
| warm-up | 5.85 | 26.5 | render/thread/ms 53.78, render/busy/ms 52.62, stage/render/prepare_assets 34.31, main/busy/ms 30.00, stage/main/update 15.21, encode//sum 14.12, stage/render/render 10.19, stage/main/post_update 8.91, render/graph/ms 8.61, stage/render/graph_submit 6.65, stream/drive/ms 4.03, encode/submit 3.47 |
| warm-up | 5.93 | 36.7 | render/thread/ms 45.40, render/busy/ms 43.17, main/busy/ms 41.61, encode//sum 26.47, stage/main/update 21.38, stage/render/render 21.37, render/graph/ms 18.00, stage/main/post_update 14.52, stage/render/graph_submit 13.62, stage/render/prepare_assets 10.01, encode/submit 7.70, encode/finish_wall 6.06 |
| warm-up | 5.97 | 35.5 | render/thread/ms 55.70, render/busy/ms 53.70, main/busy/ms 39.95, encode//sum 25.93, stage/render/render 19.21, stage/main/post_update 18.24, stage/render/prepare_assets 18.19, render/graph/ms 16.24, stage/main/update 14.77, stage/render/graph_submit 11.96, encode/submit 6.03, encode/finish_wall 6.02 |
| warm-up | 6.01 | 38.7 | render/thread/ms 54.57, render/busy/ms 53.07, main/busy/ms 38.52, stage/render/prepare_assets 26.08, encode//sum 19.51, stage/main/update 17.40, stage/render/render 15.10, stage/main/post_update 13.76, render/graph/ms 13.20, stage/render/graph_submit 10.16, encode/submit 5.51, encode/finish_wall 4.75 |
| warm-up | 6.04 | 34.7 | render/thread/ms 51.25, render/busy/ms 49.79, main/busy/ms 44.27, stage/render/prepare_assets 28.43, stage/main/update 28.10, encode//sum 19.35, stream/drive/ms 15.16, stage/render/render 14.11, render/graph/ms 11.97, stage/main/post_update 10.10, pipelines/compile_ms 9.13, stage/render/graph_submit 8.82 |
| warm-up | 6.30 | 29.0 | render/thread/ms 43.75, render/busy/ms 42.56, main/busy/ms 27.73, stage/render/prepare_assets 26.91, encode//sum 18.24, stage/main/post_update 11.98, stage/main/update 11.23, stage/render/render 10.76, render/graph/ms 8.93, stage/render/graph_submit 6.73, encode/finish_wall 3.94, encode/submit 2.84 |
| warm-up | 6.60 | 29.5 | render/thread/ms 42.66, render/busy/ms 40.84, main/busy/ms 40.33, encode//sum 25.81, stage/render/render 19.37, stage/main/post_update 17.38, stage/main/update 17.24, render/graph/ms 16.77, stage/render/graph_submit 13.17, stage/render/prepare_assets 11.98, stream/drive/ms 10.58, encode/finish_wall 6.71 |
| warm-up | 6.64 | 39.6 | render/thread/ms 49.43, render/busy/ms 48.04, main/busy/ms 36.95, stage/render/prepare_assets 27.56, encode//sum 20.38, stage/main/update 15.95, stage/main/post_update 14.75, stage/render/render 12.79, render/graph/ms 10.88, stream/drive/ms 10.86, stage/render/graph_submit 8.03, encode/finish_wall 4.75 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_off\balmora@2560x1440_off=cascadeencoders_cascadehold_meshslabs-2 --bench-baseline none --off cascadeencoders,cascadehold,meshslabs
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 18%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 92%) · 2580 MHz (min 2580) · memory 8001 MHz · 95.4 W · 74 °C (max 76) · VRAM 5674 MiB · throttled 100% of samples
