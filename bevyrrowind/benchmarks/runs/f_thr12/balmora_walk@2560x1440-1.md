# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5592 frames over 65.7 s · **median 11.46 ms (87 fps)** · mean 11.74 · p95 14.10 · p99 16.05 (1% low 62 fps) · worst 31.66 ms · 31 spikes over 1.5× median · 20.1 stutters/min (work 22.8/min)
- warm-up: 324 frames over 9.3 s, worst 2518.4 ms
- hitches (> 28.7 ms): **2** measured, 38 in warm-up
- timed GPU passes: 7.36 ms of a 11.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.46 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- pass worst frame 31.66 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.063 | 2.075 | 2.108 | 0.99 | 0.174 |
| main_opaque_pass_3d | 1.636 | 1.645 | 2.032 | 0.99 | 0.069 |
| early prepass | 0.534 | 0.537 | 0.771 | 0.99 | 0.082 |
| bloom | 0.319 | 0.321 | 0.327 | 0.99 | 0.063 |
| atmosphere_luts | 0.313 | 0.315 | 0.372 | 0.99 | 0.003 |
| water_velocity | 0.282 | 0.283 | 0.353 | 0.99 | 0.002 |
| water_sspr | 0.204 | 0.205 | 0.205 | 0.99 | 0.002 |
| ssao | 0.191 | 0.192 | 0.195 | 0.99 | 0.007 |
| uniform_allocation | 0.150 | 0.151 | 0.438 | 0.99 | 0.026 |
| per_view_shadow_pass_cascade1 | 0.148 | 0.575 | 0.652 | 0.26 | 0.045 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.147 | 0.99 | 0.001 |
| clouds_trace | 0.141 | 0.142 | 0.234 | 0.99 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.139 | 0.99 | 0.003 |
| upscaling | 0.117 | 0.118 | 0.126 | 0.99 | 0.005 |
| auxiliary_views | 0.117 | 0.117 | 0.506 | 0.99 | 0.114 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 0.99 | 0.002 |
| bin_unpacking | 0.103 | 0.103 | 0.126 | 0.99 | 0.018 |
| tonemapping | 0.086 | 0.086 | 0.090 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.083 | 0.084 | 0.160 | 0.99 | 0.008 |
| clustering | 0.065 | 0.065 | 0.068 | 0.99 | 0.019 |
| render_sky | 0.052 | 0.052 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.421 | 0.420 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.043 | 0.058 | 0.057 | 0.75 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.043 | 0.99 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.228 | 0.236 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.99 | 0.111 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.041 | 0.049 | 0.01 | 0.014 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.044 | 0.048 | 0.00 | 0.015 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 12.679 | 19.382 | 63.65 | 70901.8 |
| render/thread/ms | 10.480 | 12.639 | 24.86 | 58601.7 |
| render/busy/ms | 9.913 | 12.040 | 24.01 | 55436.2 |
| main/busy/ms | 9.009 | 11.217 | 29.34 | 50380.0 |
| stage/render/render | 6.720 | 8.223 | 16.39 | 37578.9 |
| render/graph/ms | 5.966 | 7.400 | 15.19 | 33360.8 |
| stage/main/post_update | 5.593 | 7.513 | 10.04 | 31275.1 |
| stage/render/graph_submit | 4.817 | 5.909 | 10.47 | 26935.8 |
| encode/finish_wall | 2.540 | 3.225 | 4.95 | 14204.7 |
| encode/submit | 2.328 | 3.099 | 5.82 | 13015.5 |
| stage/main/update | 2.167 | 2.622 | 23.12 | 12119.5 |
| stage/render/camera_driver | 1.093 | 1.511 | 4.71 | 6111.5 |
| stage/render/prepare_write_phase_buffers | 0.779 | 1.059 | 2.68 | 4358.1 |
| bigspace/total/ms | 0.616 | 1.846 | 3.32 | 3443.6 |
| stage/render/prepare_assets | 0.572 | 0.727 | 10.29 | 3197.2 |
| render/present/ms | 0.475 | 0.565 | 2.62 | 2654.8 |
| stage/render/prepare_resources | 0.460 | 0.550 | 1.09 | 2573.3 |
| stage/main/first | 0.400 | 0.510 | 5.10 | 2235.2 |
| stage/render/prepare_meshes | 0.386 | 0.494 | 1.55 | 2157.9 |
| stage/main/pre_update | 0.329 | 0.466 | 1.32 | 1839.9 |
| terrain/update/ms | 0.319 | 0.429 | 2.01 | 1786.0 |
| terrain/select/ms | 0.284 | 0.384 | 0.88 | 1588.0 |
| stage/render/specialize | 0.283 | 0.415 | 2.30 | 1584.0 |
| controller/feed/ms | 0.277 | 0.348 | 0.73 | 1547.0 |
| stage/render/prepare_bind_groups | 0.266 | 0.452 | 2.65 | 1487.2 |
| stream/drive/ms | 0.257 | 0.336 | 20.98 | 1438.5 |
| bigspace/high_precision/ms | 0.246 | 0.746 | 2.97 | 1374.4 |
| stage/main/fixed_main_loop | 0.225 | 0.365 | 1.04 | 1260.4 |
| stage/render/queue | 0.215 | 0.368 | 2.78 | 1205.0 |
| bigspace/low_precision/ms | 0.212 | 0.582 | 2.88 | 1187.7 |
| stream/tick/ms | 0.199 | 0.267 | 1.39 | 1111.8 |
| far/select/ms | 0.198 | 0.268 | 0.55 | 1106.0 |
| anim/evaluate/ms | 0.197 | 0.313 | 2.90 | 1104.2 |
| stage/render/create_views | 0.184 | 0.244 | 0.87 | 1030.0 |
| stage/render/prepare_flush | 0.179 | 0.348 | 1.01 | 1001.7 |
| stage/render/extract_commands | 0.179 | 0.210 | 0.40 | 998.5 |
| stage/render/prepare_batch_phases | 0.157 | 0.218 | 0.48 | 878.7 |
| anim/evaluate_cpu/ms | 0.148 | 0.189 | 0.45 | 827.0 |
| stage/render/prepare_views | 0.132 | 0.167 | 0.36 | 740.9 |
| controller/move/ms | 0.129 | 0.260 | 0.53 | 723.7 |
| stage/main/last | 0.120 | 0.191 | 1.85 | 672.8 |
| render/acquire/ms | 0.091 | 0.118 | 0.28 | 510.7 |
| stage/render/phase_sort | 0.077 | 0.101 | 0.35 | 428.0 |
| pipelines/compile_ms | 0.053 | 0.000 | 120.02 | 294.4 |
| pipelines/front_ms | 0.050 | 0.000 | 116.01 | 282.3 |
| physics/step/ms | 0.050 | 0.084 | 0.57 | 280.5 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.072 | 0.29 | 268.3 |
| physics/insert/ms | 0.037 | 0.399 | 1.09 | 207.4 |
| encode/resolve_diagnostics/sum | 0.035 | 0.047 | 0.41 | 196.5 |
| stage/render/cleanup | 0.031 | 0.094 | 0.70 | 175.4 |
| weather/map/ms | 0.027 | 0.034 | 0.24 | 149.9 |
| encode/camera_driver_span_begin/sum | 0.018 | 0.024 | 0.43 | 98.3 |
| encode/camera_driver_span_end/sum | 0.013 | 0.021 | 0.24 | 74.3 |
| anim/apply/ms | 0.012 | 0.022 | 0.28 | 66.5 |
| weather/eval/ms | 0.011 | 0.014 | 0.19 | 60.1 |
| controller/avoid/ms | 0.011 | 0.014 | 0.15 | 59.0 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 40.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.02 | 29.2 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 28.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 26.3 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 25.1 |
| controller/write/ms | 0.004 | 0.005 | 0.05 | 19.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.69 | 14.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.25 | 12.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 19 | 21 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1277 | 2262 | 593 |
| crowd/c0/meshes/count | count | 352 | 646 | 156 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5603 |
| crowd/c1/meshes/count | count | 1166 | 1281 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4604 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.25 | 3.76 | 2.12 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.43 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.24 | 0.01 |
| encode/finish_critical | ms | 2.25 | 3.76 | 2.12 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.41 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.81 | 29.34 | 9.64 |
| gpu/timed/ms | ms | 8.57 | 11.93 | 9.64 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 824 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 824 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1288 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3439 | 5143 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1288 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3439 | 5143 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 201 | 4 |
| phases/shadow/bins | count | 4392 | 5502 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.2 k | 12.5 k | 5442 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 194 | 239 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 192 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 298 | 299 | 299 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 563 | 9 |
| preprocess/bind_groups_created | count | 14 | 563 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 227 | 120 |
| process/cpu_usage | % | 27.88 | 46.63 | 35.58 |
| process/mem_usage | GiB | 5.32 | 5.48 | 5.48 |
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
| started_load_count |  | 4914 | 5008 | 5008 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.92 | 49.61 | 33.70 |
| system/mem_usage | % | 50.34 | 50.95 | 50.86 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 65 | 250 | 54 |
| views/all/bins | count | 7063 | 8384 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.3 k | 24.9 k | 13.1 k |
| views/all/pipelines | count | 239 | 288 | 65 |
| views/all/pipelines_distinct | count | 71 | 75 | 61 |
| views/all/work_items | count | 10.8 k | 22.2 k | 8165 |
| views/cascade/batch_sets | count | 10 | 33 | 4 |
| views/cascade/bins | count | 2218 | 2883 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7671 | 9649 | 5442 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 516 | 8952 | 239 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2167 | 2784 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2414 | 3086 | 0 |
| views/lamp_face/pipelines | count | 170 | 214 | 0 |
| views/lamp_face/work_items | count | 0 | 2917 | 0 |
| views/main/batch_sets | count | 51 | 53 | 50 |
| views/main/bins | count | 2882 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8521 | 13.0 k | 7618 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8674 | 13.3 k | 7926 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 57.06 | 281 | 319.1 k |
| anim/events/count | count | 0.09 | 3 | 478 |
| assets/Image/added | count | 0.11 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.73 | 152 | 4061 |
| assets/Mesh/removed | count | 0.87 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.87 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.84 | 1485 | 4675 |
| encode//count | count | 37.86 | 272 | 211.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5592 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5592 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5592 |
| physics/inserts | count | 0.40 | 101 | 2264 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 161 | 2335 |
| physics/steps | count | 0.70 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 27 | 378 |
| stream/upload/bytes | bytes | 6054 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1327 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.7 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2518.4 | main/busy/ms 2491, stage/main/post_update 2390, stage/main/update 94.55, surface/advance_cpu_ms 65.86, stage/main/pre_update 2.34, stage/main/last 1.01, surface/cell_upload_ms 0.70, stream/drive/ms 0.66, stage/main/first 0.54, stage/main/fixed_main_loop 0.52, stream/tick/ms 0.19, bigspace/total/ms 0.13 |
| warm-up | 3.27 | 752.0 | main/busy/ms 2627, stage/main/post_update 2414, render/thread/ms 722, render/busy/ms 555, stage/render/prepare_assets 518, stage/main/update 202, stage/render/prepare_views 164, render/acquire/ms 163, surface/advance_cpu_ms 65.86, stage/render/prepare_resources 21.67, encode//sum 15.36, stage/render/render 11.96 |
| warm-up | 3.36 | 91.1 | render/thread/ms 796, render/busy/ms 626, stage/render/prepare_assets 563, stage/render/prepare_views 165, render/acquire/ms 164, main/busy/ms 163, pipelines/compile_ms 151, pipelines/front_ms 137, stage/main/update 114, stage/main/post_update 37.26, stage/render/render 28.70, stage/render/prepare_resources 27.41 |
| warm-up | 3.40 | 39.2 | pipelines/compile_ms 304, pipelines/front_ms 280, pipelines/wait_ms 131, render/thread/ms 111, render/busy/ms 98.35, main/busy/ms 64.72, stage/render/prepare_assets 47.01, stage/main/post_update 44.48, stage/render/render 43.88, render/graph/ms 30.95, terrain/update/ms 25.52, pipelines/driver_ms 24.13 |
| warm-up | 3.48 | 82.2 | pipelines/wait_ms 1790, pipelines/compile_ms 703, pipelines/front_ms 661, render/thread/ms 116, main/busy/ms 109, render/busy/ms 105, stage/main/post_update 65.96, pipelines/driver_ms 41.95, stage/render/prepare_resources 34.38, stage/render/render 30.82, stage/main/update 30.36, stage/render/prepare_bind_groups 23.25 |
| warm-up | 3.64 | 155.4 | pipelines/wait_ms 5837, pipelines/compile_ms 2467, pipelines/front_ms 2354, render/thread/ms 233, render/busy/ms 230, main/busy/ms 127, pipelines/driver_ms 113, stage/main/update 69.56, stage/render/prepare_resources 61.91, stage/render/prepare_write_phase_buffers 52.12, stage/main/post_update 45.52, stage/render/prepare_assets 44.59 |
| warm-up | 3.67 | 34.1 | pipelines/wait_ms 4276, pipelines/compile_ms 2374, pipelines/front_ms 2269, render/thread/ms 185, render/busy/ms 182, main/busy/ms 119, pipelines/driver_ms 105, stage/main/update 89.55, stage/render/prepare_write_phase_buffers 56.67, stage/render/prepare_assets 43.29, stage/render/render 42.87, render/graph/ms 38.88 |
| warm-up | 3.88 | 208.4 | pipelines/compile_ms 1873, pipelines/front_ms 1716, pipelines/wait_ms 625, render/thread/ms 202, render/busy/ms 158, pipelines/driver_ms 157, stage/render/prepare_assets 128, main/busy/ms 88.25, stage/render/render 59.67, stage/main/update 52.74, render/present/ms 43.32, stage/main/post_update 19.41 |
| warm-up | 3.93 | 39.2 | pipelines/compile_ms 800, pipelines/front_ms 605, pipelines/driver_ms 195, render/thread/ms 47.40, render/busy/ms 45.23, main/busy/ms 32.75, encode//sum 24.82, stage/render/render 17.60, stage/main/post_update 15.99, render/graph/ms 15.28, stage/render/prepare_assets 15.06, stage/render/graph_submit 12.91 |
| warm-up | 4.08 | 86.6 | pipelines/compile_ms 879, pipelines/front_ms 838, render/thread/ms 65.30, main/busy/ms 63.56, render/busy/ms 62.52, stage/main/update 46.47, stage/render/prepare_assets 45.18, stream/drive/ms 41.38, pipelines/driver_ms 40.91, stage/render/render 12.85, stage/main/post_update 12.31, render/graph/ms 9.81 |
| warm-up | 4.12 | 36.2 | pipelines/compile_ms 472, pipelines/front_ms 456, render/thread/ms 76.34, render/busy/ms 74.11, stage/render/prepare_assets 40.13, main/busy/ms 37.22, stage/main/update 19.66, pipelines/driver_ms 15.85, stream/drive/ms 14.17, stage/render/render 12.89, render/graph/ms 10.34, stage/render/prepare_resources 9.99 |
| warm-up | 4.15 | 29.3 | pipelines/compile_ms 330, pipelines/front_ms 318, render/thread/ms 55.24, render/busy/ms 53.31, main/busy/ms 37.30, stage/main/update 20.18, stage/render/prepare_assets 20.07, stream/drive/ms 14.32, stage/render/render 12.16, pipelines/driver_ms 12.05, stage/render/prepare_resources 9.96, render/graph/ms 9.86 |
| warm-up | 4.20 | 49.1 | pipelines/compile_ms 492, pipelines/front_ms 475, render/thread/ms 68.98, render/busy/ms 66.94, main/busy/ms 46.57, stage/render/prepare_assets 43.34, stage/main/update 31.42, stream/drive/ms 25.70, pipelines/driver_ms 17.35, stage/render/render 15.80, render/graph/ms 13.28, stage/render/graph_submit 10.81 |
| warm-up | 4.26 | 65.0 | pipelines/compile_ms 342, pipelines/front_ms 330, render/thread/ms 95.82, render/busy/ms 93.82, main/busy/ms 63.87, stage/render/prepare_assets 55.85, stage/main/update 31.96, stream/drive/ms 27.25, stage/main/post_update 24.30, stage/render/render 14.76, stage/render/prepare_write_phase_buffers 12.63, pipelines/driver_ms 12.24 |
| warm-up | 4.35 | 92.9 | pipelines/compile_ms 1212, pipelines/front_ms 1147, render/thread/ms 118, render/busy/ms 116, main/busy/ms 98.48, stage/render/prepare_assets 68.38, pipelines/driver_ms 64.99, stage/main/update 56.14, stage/main/post_update 27.90, stage/render/prepare_write_phase_buffers 25.78, stage/render/render 9.92, stage/main/pre_update 8.20 |
| warm-up | 4.54 | 182.1 | pipelines/compile_ms 3463, pipelines/front_ms 3204, pipelines/driver_ms 259, render/thread/ms 197, render/busy/ms 186, stage/render/prepare_assets 96.28, main/busy/ms 91.91, stage/main/update 59.11, stage/render/prepare_bind_groups 49.80, pipelines/wait_ms 34.30, stage/render/render 23.30, stage/main/post_update 19.48 |
| warm-up | 4.63 | 91.8 | pipelines/compile_ms 3269, pipelines/front_ms 3026, pipelines/wait_ms 385, pipelines/driver_ms 243, render/thread/ms 218, render/busy/ms 208, stage/render/prepare_assets 93.01, main/busy/ms 60.81, stage/render/prepare_bind_groups 52.27, stage/render/render 33.71, render/graph/ms 22.66, stage/main/post_update 19.84 |
| warm-up | 4.70 | 75.1 | pipelines/compile_ms 1890, pipelines/front_ms 1814, pipelines/wait_ms 464, render/thread/ms 151, render/busy/ms 149, pipelines/driver_ms 75.80, main/busy/ms 70.54, stage/render/prepare_assets 55.68, stage/render/render 24.81, stage/render/prepare_write_phase_buffers 24.28, render/graph/ms 22.43, stage/render/prepare_resources 20.13 |
| warm-up | 4.78 | 73.2 | pipelines/compile_ms 2624, pipelines/front_ms 2529, pipelines/wait_ms 1225, render/thread/ms 126, render/busy/ms 124, pipelines/driver_ms 94.99, main/busy/ms 61.86, stage/render/prepare_assets 38.75, stage/main/post_update 23.92, stage/render/render 23.66, encode//sum 23.62, render/graph/ms 20.79 |
| warm-up | 4.83 | 48.1 | pipelines/compile_ms 1899, pipelines/front_ms 1820, pipelines/wait_ms 1208, render/thread/ms 109, encode//sum 106, render/busy/ms 105, pipelines/driver_ms 79.63, main/busy/ms 51.86, stage/render/render 37.43, render/graph/ms 34.11, stage/main/post_update 29.01, stage/render/graph_submit 25.75 |
| warm-up | 4.86 | 35.5 | pipelines/compile_ms 433, pipelines/front_ms 412, pipelines/wait_ms 241, encode//sum 97.58, render/thread/ms 74.23, render/busy/ms 70.44, main/busy/ms 37.32, stage/render/render 30.92, render/graph/ms 28.00, stage/render/graph_submit 21.34, pipelines/driver_ms 20.67, stage/render/prepare_assets 17.44 |
| warm-up | 4.89 | 31.3 | pipelines/compile_ms 599, pipelines/front_ms 583, pipelines/wait_ms 320, main/busy/ms 77.15, stage/main/update 58.82, render/thread/ms 52.45, stream/drive/ms 50.57, render/busy/ms 50.42, stage/render/prepare_assets 25.86, pipelines/driver_ms 16.39, stage/render/render 15.49, render/graph/ms 12.69 |
| warm-up | 4.98 | 88.1 | pipelines/compile_ms 1546, pipelines/front_ms 1501, pipelines/wait_ms 651, main/busy/ms 127, stage/main/update 85.69, render/thread/ms 62.90, render/busy/ms 60.18, stream/drive/ms 51.24, pipelines/driver_ms 44.28, stage/main/post_update 31.43, stage/render/render 23.61, render/graph/ms 19.97 |
| warm-up | 5.08 | 99.9 | pipelines/compile_ms 1583, pipelines/front_ms 1537, pipelines/wait_ms 477, main/busy/ms 154, stage/main/update 114, render/thread/ms 94.59, render/busy/ms 91.99, stream/drive/ms 57.54, pipelines/driver_ms 45.74, stage/render/prepare_assets 37.51, stage/main/post_update 30.69, stage/render/render 25.33 |
| warm-up | 5.19 | 113.8 | pipelines/compile_ms 1690, pipelines/front_ms 1638, main/busy/ms 125, render/thread/ms 112, render/busy/ms 110, stage/main/update 102, stage/render/prepare_assets 75.11, stream/drive/ms 57.51, pipelines/driver_ms 51.87, encode//sum 17.54, stage/render/render 15.60, stage/main/post_update 13.56 |
| warm-up | 5.28 | 84.7 | pipelines/compile_ms 1934, pipelines/front_ms 1875, render/thread/ms 132, render/busy/ms 130, stage/render/prepare_assets 99.49, main/busy/ms 95.87, stage/main/update 71.73, pipelines/driver_ms 59.53, stream/drive/ms 41.64, stage/render/render 20.37, render/graph/ms 18.25, encode//sum 17.25 |
| warm-up | 5.32 | 40.6 | pipelines/compile_ms 707, pipelines/front_ms 687, render/thread/ms 104, render/busy/ms 102, main/busy/ms 79.01, stage/render/prepare_assets 71.85, stage/main/update 61.96, stream/drive/ms 41.33, pipelines/driver_ms 20.46, stage/render/render 19.01, render/graph/ms 16.83, encode//sum 15.92 |
| warm-up | 5.35 | 29.1 | pipelines/compile_ms 87.48, pipelines/front_ms 83.00, main/busy/ms 65.83, render/thread/ms 54.20, render/busy/ms 51.86, stage/main/update 51.85, stream/drive/ms 34.79, stage/render/prepare_assets 19.17, encode//sum 17.43, stage/render/render 14.61, render/graph/ms 11.82, stage/render/graph_submit 8.62 |
| warm-up | 5.39 | 44.5 | pipelines/compile_ms 388, pipelines/front_ms 375, main/busy/ms 59.44, stage/main/update 44.59, render/thread/ms 44.58, render/busy/ms 42.34, stream/drive/ms 34.43, encode//sum 16.45, stage/render/render 14.41, pipelines/driver_ms 12.53, stage/render/prepare_assets 12.11, render/graph/ms 11.65 |
| warm-up | 5.52 | 32.1 | pipelines/compile_ms 217, pipelines/front_ms 210, render/thread/ms 50.45, render/busy/ms 48.00, main/busy/ms 47.83, stage/main/update 28.30, stage/render/render 21.00, encode//sum 18.24, render/graph/ms 17.80, stream/drive/ms 14.91, stage/render/graph_submit 13.32, stage/main/post_update 13.16 |
| warm-up | 5.57 | 48.8 | pipelines/compile_ms 337, pipelines/front_ms 326, render/thread/ms 62.37, render/busy/ms 60.24, main/busy/ms 49.11, stage/render/prepare_assets 30.39, stage/main/update 30.29, stage/render/render 19.43, encode//sum 17.93, render/graph/ms 16.50, stream/drive/ms 13.84, stage/main/post_update 12.87 |
| warm-up | 5.61 | 36.8 | pipelines/compile_ms 400, pipelines/front_ms 387, render/thread/ms 63.87, render/busy/ms 61.95, main/busy/ms 48.84, stage/render/prepare_assets 33.23, stage/main/update 25.12, stage/render/render 17.48, stage/main/post_update 17.46, render/graph/ms 15.15, stream/drive/ms 14.56, encode//sum 14.17 |
| warm-up | 5.64 | 35.3 | pipelines/compile_ms 507, pipelines/front_ms 491, render/thread/ms 58.06, render/busy/ms 56.09, main/busy/ms 50.38, stage/render/prepare_assets 24.78, stage/main/update 23.47, stage/main/post_update 19.87, stage/render/render 17.80, pipelines/driver_ms 16.06, encode//sum 16.05, render/graph/ms 15.35 |
| warm-up | 5.67 | 32.1 | pipelines/compile_ms 866, pipelines/front_ms 838, render/thread/ms 57.04, render/busy/ms 55.45, main/busy/ms 33.71, stage/render/prepare_assets 32.12, pipelines/driver_ms 27.49, encode//sum 18.09, stage/main/post_update 15.10, stage/render/render 13.33, stage/main/update 11.92, render/graph/ms 11.04 |
| warm-up | 6.37 | 34.6 | render/thread/ms 40.49, render/busy/ms 39.33, main/busy/ms 24.43, encode//sum 19.44, stage/render/prepare_assets 16.59, stage/main/post_update 9.91, stage/render/render 9.47, stage/main/update 9.05, render/graph/ms 7.82, stage/render/graph_submit 5.56, stage/render/specialize 3.85, encode/finish_wall 3.41 |
| warm-up | 6.58 | 29.5 | render/thread/ms 46.87, render/busy/ms 45.15, main/busy/ms 38.30, encode//sum 22.89, stage/main/update 16.87, stage/render/render 15.98, stage/main/post_update 14.59, render/graph/ms 13.64, stage/render/graph_submit 9.97, stage/render/prepare_assets 9.83, encode/submit 5.44, stream/drive/ms 5.20 |
| warm-up | 6.70 | 29.6 | main/busy/ms 38.20, render/thread/ms 37.54, render/busy/ms 36.40, encode//sum 20.84, stage/main/update 19.40, stage/main/post_update 13.73, stage/render/prepare_assets 12.91, stage/render/render 11.89, render/graph/ms 9.76, stage/render/graph_submit 6.64, encode/finish_wall 3.71, stream/drive/ms 3.62 |
| warm-up | 6.88 | 50.0 | render/thread/ms 52.93, render/busy/ms 50.98, main/busy/ms 41.39, stage/render/prepare_assets 30.15, stage/main/update 27.60, encode//sum 20.87, stream/drive/ms 19.42, stage/render/render 13.42, render/graph/ms 11.04, stage/main/post_update 9.71, stage/render/graph_submit 8.72, encode/submit 4.86 |
| measure | 18.18 | 31.7 | main/busy/ms 39.88, encode//sum 25.77, stage/main/update 25.54, stream/drive/ms 21.21, render/thread/ms 21.17, render/busy/ms 20.12, stage/render/render 12.53, stage/main/post_update 12.03, render/graph/ms 11.02, stage/render/graph_submit 8.54, encode/submit 4.45, encode/finish_wall 4.17 |
| measure | 59.05 | 29.7 | render/thread/ms 40.87, render/busy/ms 39.70, main/busy/ms 26.97, encode//sum 23.19, stage/render/prepare_assets 15.64, stage/render/render 13.39, stage/main/post_update 13.21, render/graph/ms 11.93, stage/render/graph_submit 9.25, stage/main/update 7.66, encode/finish_wall 4.83, encode/submit 4.50 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_thr12\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_COMPUTE_THREADS=12
- before the run: GPU 0%, CPU 8%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **76%** (min 54%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.9 W · 71 °C (max 76) · VRAM 5248 MiB · throttled 100% of samples
