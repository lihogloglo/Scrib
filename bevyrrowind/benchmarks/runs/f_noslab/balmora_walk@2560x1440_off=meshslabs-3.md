# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5673 frames over 65.6 s · **median 11.31 ms (88 fps)** · mean 11.57 · p95 14.13 · p99 16.08 (1% low 62 fps) · worst 62.73 ms · 36 spikes over 1.5× median · 27.4 stutters/min (work 22.9/min)
- warm-up: 323 frames over 8.9 s, worst 2500.0 ms
- hitches (> 28.3 ms): **2** measured, 33 in warm-up
- timed GPU passes: 7.18 ms of a 11.57 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.31 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- **FAIL** worst frame 62.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.991 | 2.073 | 2.111 | 0.96 | 0.173 |
| main_opaque_pass_3d | 1.577 | 1.642 | 2.054 | 0.96 | 0.055 |
| early prepass | 0.539 | 0.561 | 1.122 | 0.96 | 0.079 |
| bloom | 0.308 | 0.321 | 0.327 | 0.96 | 0.063 |
| atmosphere_luts | 0.303 | 0.315 | 0.374 | 0.96 | 0.003 |
| water_velocity | 0.273 | 0.284 | 0.353 | 0.96 | 0.001 |
| water_sspr | 0.197 | 0.206 | 0.206 | 0.96 | 0.002 |
| auxiliary_views | 0.193 | 0.201 | 0.691 | 0.96 | 0.172 |
| ssao | 0.185 | 0.192 | 0.195 | 0.96 | 0.007 |
| uniform_allocation | 0.143 | 0.149 | 0.417 | 0.96 | 0.023 |
| atmos_froxel_inject | 0.141 | 0.147 | 0.147 | 0.96 | 0.001 |
| clouds_trace | 0.137 | 0.142 | 0.237 | 0.96 | 0.001 |
| per_view_shadow_pass_cascade1 | 0.130 | 0.576 | 0.648 | 0.22 | 0.040 |
| water_fft | 0.128 | 0.134 | 0.140 | 0.96 | 0.003 |
| upscaling | 0.114 | 0.119 | 0.129 | 0.96 | 0.004 |
| clouds_composite | 0.103 | 0.108 | 0.110 | 0.96 | 0.002 |
| bin_unpacking | 0.099 | 0.103 | 0.127 | 0.96 | 0.017 |
| tonemapping | 0.083 | 0.086 | 0.090 | 0.96 | 0.003 |
| main_transmissive_pass_3d | 0.081 | 0.084 | 0.161 | 0.96 | 0.008 |
| clustering | 0.062 | 0.065 | 0.068 | 0.96 | 0.018 |
| render_sky | 0.050 | 0.052 | 0.053 | 0.96 | 0.002 |
| clouds_resolve | 0.047 | 0.049 | 0.050 | 0.96 | 0.001 |
| surface_trail | 0.041 | 0.057 | 0.057 | 0.71 | 0.002 |
| lightprobe_irradiance_map | 0.039 | 0.417 | 0.421 | 0.09 | 0.001 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.030 | 0.042 | 0.96 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.031 | 0.96 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.96 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.223 | 0.233 | 0.09 | 0.004 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.025 | 0.96 | 0.111 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.96 | 0.003 |
| per_view_shadow_pass | 0.009 | 0.010 | 0.015 | 0.96 | 0.004 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.96 | 0.002 |
| lightprobe_copy | 0.001 | 0.014 | 0.014 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.024 | 0.048 | 0.01 | 0.010 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.017 | 0.017 | 0.00 | 0.011 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 11.218 | 17.739 | 47.86 | 63638.8 |
| render/thread/ms | 10.297 | 12.730 | 61.70 | 58415.0 |
| render/busy/ms | 9.714 | 12.160 | 60.93 | 55105.9 |
| main/busy/ms | 9.208 | 11.453 | 28.96 | 52234.9 |
| stage/render/render | 6.662 | 8.435 | 58.80 | 37791.1 |
| render/graph/ms | 5.892 | 7.631 | 57.86 | 33426.0 |
| stage/main/post_update | 5.822 | 7.755 | 11.16 | 33028.6 |
| stage/render/graph_submit | 4.736 | 6.091 | 56.87 | 26869.2 |
| encode/finish_wall | 2.436 | 3.164 | 5.35 | 13817.8 |
| encode/submit | 2.376 | 3.256 | 54.91 | 13479.9 |
| stage/main/update | 2.151 | 2.601 | 22.47 | 12200.6 |
| stage/render/camera_driver | 1.103 | 1.514 | 3.90 | 6255.0 |
| stage/render/prepare_write_phase_buffers | 0.756 | 1.054 | 2.39 | 4288.7 |
| bigspace/total/ms | 0.630 | 2.160 | 3.63 | 3573.4 |
| stage/render/prepare_assets | 0.552 | 0.705 | 9.89 | 3129.3 |
| render/present/ms | 0.493 | 0.611 | 2.18 | 2794.3 |
| stage/main/first | 0.394 | 0.517 | 4.84 | 2235.3 |
| stage/render/prepare_resources | 0.383 | 0.481 | 0.96 | 2174.9 |
| stage/render/prepare_meshes | 0.382 | 0.492 | 1.94 | 2166.6 |
| stage/main/pre_update | 0.327 | 0.496 | 1.48 | 1853.9 |
| terrain/update/ms | 0.303 | 0.391 | 1.62 | 1720.4 |
| stage/render/specialize | 0.285 | 0.422 | 2.45 | 1618.2 |
| terrain/select/ms | 0.270 | 0.347 | 0.80 | 1530.6 |
| controller/feed/ms | 0.268 | 0.341 | 1.28 | 1522.8 |
| bigspace/low_precision/ms | 0.260 | 0.952 | 3.04 | 1477.6 |
| stage/render/prepare_bind_groups | 0.253 | 0.428 | 2.34 | 1437.4 |
| stream/drive/ms | 0.250 | 0.331 | 20.00 | 1416.0 |
| stage/render/queue | 0.248 | 0.408 | 2.53 | 1404.6 |
| stage/main/fixed_main_loop | 0.220 | 0.359 | 0.96 | 1249.1 |
| bigspace/high_precision/ms | 0.214 | 1.077 | 3.01 | 1211.2 |
| anim/evaluate/ms | 0.212 | 0.379 | 2.74 | 1205.2 |
| stream/tick/ms | 0.193 | 0.258 | 0.73 | 1092.8 |
| far/select/ms | 0.187 | 0.248 | 0.52 | 1060.6 |
| stage/render/prepare_flush | 0.170 | 0.344 | 0.87 | 966.0 |
| stage/render/create_views | 0.168 | 0.220 | 1.00 | 952.0 |
| stage/render/extract_commands | 0.164 | 0.195 | 0.41 | 931.7 |
| stage/render/prepare_batch_phases | 0.159 | 0.221 | 0.47 | 902.5 |
| anim/evaluate_cpu/ms | 0.146 | 0.191 | 0.55 | 830.1 |
| stage/render/prepare_views | 0.132 | 0.167 | 0.46 | 748.5 |
| controller/move/ms | 0.122 | 0.222 | 0.51 | 690.8 |
| stage/main/last | 0.119 | 0.186 | 1.56 | 674.8 |
| stage/render/phase_sort | 0.092 | 0.140 | 0.40 | 521.4 |
| render/acquire/ms | 0.091 | 0.118 | 0.41 | 514.7 |
| pipelines/compile_ms | 0.052 | 0.000 | 123.96 | 293.5 |
| pipelines/front_ms | 0.050 | 0.000 | 119.97 | 281.7 |
| physics/step/ms | 0.048 | 0.082 | 0.43 | 272.3 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.30 | 267.5 |
| physics/insert/ms | 0.033 | 0.368 | 0.90 | 188.7 |
| stage/render/cleanup | 0.032 | 0.100 | 0.82 | 179.3 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.47 | 173.0 |
| weather/map/ms | 0.026 | 0.034 | 0.08 | 145.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.26 | 82.8 |
| anim/apply/ms | 0.011 | 0.020 | 0.25 | 64.2 |
| weather/eval/ms | 0.011 | 0.014 | 0.20 | 60.0 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.27 | 59.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.19 | 58.7 |
| script/update/ms | 0.007 | 0.009 | 0.12 | 38.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.11 | 28.6 |
| script/ui/ms | 0.005 | 0.006 | 0.05 | 28.3 |
| script/input/ms | 0.005 | 0.006 | 0.08 | 26.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 25.7 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 19.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.79 | 13.9 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.98 | 11.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| crowd/c0/count | count | 26 | 42 | 15 |
| crowd/c0/joints/count | count | 1301 | 2218 | 714 |
| crowd/c0/meshes/count | count | 353 | 633 | 198 |
| crowd/c1/count | count | 146 | 156 | 146 |
| crowd/c1/joints/count | count | 5527 | 5982 | 5482 |
| crowd/c1/meshes/count | count | 1165 | 1284 | 1143 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4608 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.01 | 3.62 | 1.90 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.26 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.27 | 0.01 |
| encode/finish_critical | ms | 2.01 | 3.62 | 1.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.47 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.78 | 60.93 | 9.18 |
| gpu/timed/ms | ms | 8.64 | 12.06 | 9.18 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1287 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3404 | 5101 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1287 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3404 | 5101 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 205 | 4 |
| phases/shadow/bins | count | 4392 | 5661 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.2 k | 12.6 k | 5438 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 191 | 238 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 183 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 298 | 299 | 299 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 571 | 6 |
| preprocess/bind_groups_created | count | 14 | 571 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 240 | 123 |
| process/cpu_usage | % | 26.08 | 38.46 | 17.41 |
| process/mem_usage | GiB | 5.40 | 5.56 | 5.56 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.29 | 47.92 | 29.25 |
| system/mem_usage | % | 51.33 | 53.02 | 51.62 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 69 | 254 | 54 |
| views/all/bins | count | 6976 | 8541 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 25.0 k | 13.0 k |
| views/all/pipelines | count | 238 | 287 | 65 |
| views/all/pipelines_distinct | count | 71 | 76 | 61 |
| views/all/work_items | count | 10.8 k | 22.5 k | 8157 |
| views/cascade/batch_sets | count | 10 | 33 | 4 |
| views/cascade/bins | count | 2231 | 2892 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7661 | 9665 | 5438 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 540 | 9570 | 239 |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 2149 | 2889 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2408 | 3194 | 0 |
| views/lamp_face/pipelines | count | 167 | 208 | 0 |
| views/lamp_face/work_items | count | 0 | 2950 | 0 |
| views/main/batch_sets | count | 50 | 55 | 50 |
| views/main/bins | count | 2884 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8426 | 13.0 k | 7610 |
| views/main/pipelines | count | 47 | 51 | 46 |
| views/main/work_items | count | 8587 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 63.06 | 307 | 357.7 k |
| anim/events/count | count | 0.09 | 3 | 510 |
| assets/Image/added | count | 0.10 | 40 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.85 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.82 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.82 | 1495 | 4676 |
| encode//count | count | 41.38 | 272 | 234.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5673 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5673 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5673 |
| physics/inserts | count | 0.40 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.69 | 3 | 3937 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
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
| stream/spawn/items | count | 0.07 | 19 | 378 |
| stream/upload/bytes | bytes | 5967 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1308 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.3 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2500.0 | main/busy/ms 2474, stage/main/post_update 2376, stage/main/update 91.68, surface/advance_cpu_ms 65.34, stage/main/pre_update 2.52, stage/main/last 1.19, surface/cell_upload_ms 0.69, stage/main/first 0.58, stage/main/fixed_main_loop 0.37, stream/drive/ms 0.33, bigspace/total/ms 0.11, weather/map/ms 0.11 |
| warm-up | 3.19 | 688.0 | main/busy/ms 2614, stage/main/post_update 2401, render/thread/ms 660, render/busy/ms 497, stage/render/prepare_assets 471, stage/main/update 200, stage/render/prepare_views 162, render/acquire/ms 161, surface/advance_cpu_ms 65.34, stage/render/prepare_resources 11.88, stage/render/render 10.16, encode//sum 9.78 |
| warm-up | 3.25 | 63.0 | render/thread/ms 699, render/busy/ms 535, stage/render/prepare_assets 492, stage/render/prepare_views 162, render/acquire/ms 161, main/busy/ms 159, stage/main/update 112, stage/main/post_update 35.63, stage/render/render 19.87, terrain/update/ms 17.85, encode//sum 15.67, stage/render/prepare_resources 15.26 |
| warm-up | 3.44 | 168.8 | pipelines/wait_ms 5136, pipelines/compile_ms 1807, pipelines/front_ms 1691, render/thread/ms 158, render/busy/ms 149, pipelines/driver_ms 116, main/busy/ms 113, stage/main/post_update 87.94, stage/render/prepare_assets 51.50, stage/render/prepare_resources 37.35, stage/render/prepare_write_phase_buffers 34.16, stage/render/render 18.22 |
| warm-up | 3.48 | 37.5 | pipelines/wait_ms 5416, pipelines/compile_ms 2066, pipelines/front_ms 1935, render/thread/ms 170, render/busy/ms 167, main/busy/ms 160, pipelines/driver_ms 131, stage/main/post_update 65.40, stage/main/update 65.40, stage/render/prepare_assets 63.32, stage/render/prepare_resources 43.86, stage/render/prepare_write_phase_buffers 34.60 |
| warm-up | 3.63 | 151.5 | pipelines/compile_ms 1995, pipelines/front_ms 1881, pipelines/wait_ms 815, render/thread/ms 116, pipelines/driver_ms 115, main/busy/ms 105, render/busy/ms 97.61, stage/main/update 68.54, stage/render/prepare_assets 67.09, stage/render/render 34.44, stage/main/post_update 18.38, render/present/ms 17.90 |
| warm-up | 3.69 | 35.3 | pipelines/compile_ms 594, pipelines/front_ms 542, render/thread/ms 57.32, render/busy/ms 54.32, pipelines/driver_ms 52.11, stage/render/prepare_assets 38.51, main/busy/ms 37.97, pipelines/wait_ms 20.98, encode//sum 20.92, stage/main/update 18.69, stage/main/post_update 14.53, stage/render/render 12.04 |
| warm-up | 3.81 | 64.4 | pipelines/compile_ms 1190, pipelines/front_ms 1154, render/thread/ms 52.83, main/busy/ms 52.35, render/busy/ms 50.51, stage/main/update 42.31, stream/drive/ms 37.97, pipelines/driver_ms 36.62, stage/render/prepare_assets 32.02, stage/render/render 14.17, render/graph/ms 11.58, stage/render/graph_submit 8.11 |
| warm-up | 3.96 | 72.6 | pipelines/compile_ms 272, pipelines/front_ms 261, render/thread/ms 66.31, render/busy/ms 63.40, main/busy/ms 49.24, stage/render/prepare_assets 44.49, stage/main/update 31.45, stream/drive/ms 27.40, stage/render/render 11.41, pipelines/driver_ms 10.98, stage/main/post_update 10.52, render/graph/ms 8.86 |
| warm-up | 4.02 | 63.7 | pipelines/compile_ms 486, pipelines/front_ms 471, render/thread/ms 103, render/busy/ms 100, stage/render/prepare_assets 42.48, main/busy/ms 31.18, stage/render/prepare_flush 21.38, pipelines/driver_ms 15.75, stage/main/update 13.93, stage/render/render 12.09, render/graph/ms 9.36, stage/main/post_update 8.82 |
| warm-up | 4.13 | 107.5 | pipelines/compile_ms 1508, pipelines/front_ms 1466, render/thread/ms 135, render/busy/ms 103, main/busy/ms 57.66, stage/render/prepare_assets 47.35, pipelines/driver_ms 41.63, stage/render/render 40.94, stage/main/update 38.80, render/present/ms 32.28, stage/render/prepare_flush 21.39, stage/main/post_update 11.02 |
| warm-up | 4.19 | 60.8 | pipelines/compile_ms 2282, pipelines/front_ms 2191, render/thread/ms 123, pipelines/driver_ms 91.05, render/busy/ms 90.13, stage/render/prepare_assets 68.20, main/busy/ms 61.98, stage/render/render 42.81, stage/main/update 38.96, render/present/ms 32.24, pipelines/wait_ms 21.36, stage/main/post_update 15.23 |
| warm-up | 4.25 | 52.8 | pipelines/compile_ms 1503, pipelines/front_ms 1435, render/thread/ms 85.99, render/busy/ms 84.33, pipelines/driver_ms 68.47, pipelines/wait_ms 57.57, main/busy/ms 56.09, stage/render/prepare_assets 49.70, stage/main/post_update 24.33, stage/main/update 20.67, stage/render/render 13.80, render/graph/ms 11.64 |
| warm-up | 4.33 | 80.9 | pipelines/compile_ms 1465, pipelines/front_ms 1424, pipelines/wait_ms 276, render/thread/ms 109, render/busy/ms 107, main/busy/ms 79.36, stage/main/post_update 48.12, pipelines/driver_ms 41.66, stage/render/prepare_assets 40.01, encode//sum 21.91, stage/main/update 20.68, stage/render/render 16.24 |
| warm-up | 4.43 | 101.6 | pipelines/compile_ms 2718, pipelines/front_ms 2608, pipelines/wait_ms 1304, render/thread/ms 164, render/busy/ms 162, pipelines/driver_ms 110, main/busy/ms 99.50, encode//sum 73.95, stage/main/post_update 60.64, stage/render/prepare_resources 39.90, stage/render/prepare_write_phase_buffers 31.53, stage/render/prepare_assets 27.95 |
| warm-up | 4.50 | 70.5 | pipelines/compile_ms 2743, pipelines/front_ms 2616, pipelines/wait_ms 2405, render/thread/ms 152, render/busy/ms 150, pipelines/driver_ms 127, main/busy/ms 80.41, encode//sum 74.74, stage/main/post_update 43.11, stage/render/prepare_resources 33.95, stage/render/prepare_assets 33.70, stage/render/prepare_write_phase_buffers 31.24 |
| warm-up | 4.56 | 62.0 | pipelines/wait_ms 1647, pipelines/compile_ms 1645, pipelines/front_ms 1578, main/busy/ms 96.67, render/thread/ms 93.47, render/busy/ms 91.96, stage/main/update 67.90, pipelines/driver_ms 66.81, stream/drive/ms 59.73, stage/render/prepare_assets 52.12, encode//sum 21.69, stage/main/post_update 20.91 |
| warm-up | 4.64 | 76.4 | pipelines/compile_ms 1568, pipelines/front_ms 1515, pipelines/wait_ms 1093, main/busy/ms 110, stage/main/update 84.63, render/thread/ms 78.37, render/busy/ms 76.62, stream/drive/ms 60.42, pipelines/driver_ms 52.94, stage/render/prepare_assets 32.06, stage/render/prepare_resources 18.63, stage/render/render 14.78 |
| warm-up | 4.69 | 52.5 | pipelines/compile_ms 1864, pipelines/front_ms 1805, pipelines/wait_ms 1240, main/busy/ms 126, stage/main/update 96.61, render/thread/ms 77.74, render/busy/ms 75.71, pipelines/driver_ms 59.29, stream/drive/ms 48.39, stage/render/prepare_assets 24.12, stage/render/prepare_resources 18.90, stage/main/post_update 16.68 |
| warm-up | 4.80 | 106.6 | pipelines/compile_ms 1194, pipelines/front_ms 1158, pipelines/wait_ms 456, main/busy/ms 115, stage/main/update 87.61, render/thread/ms 83.34, render/busy/ms 81.48, stream/drive/ms 48.23, stage/render/prepare_assets 40.41, pipelines/driver_ms 35.67, encode//sum 19.24, stage/main/post_update 17.19 |
| warm-up | 4.90 | 75.6 | pipelines/compile_ms 1301, pipelines/front_ms 1264, main/busy/ms 75.65, stage/main/update 58.97, render/thread/ms 50.43, render/busy/ms 48.77, stream/drive/ms 42.70, pipelines/driver_ms 36.84, stage/render/prepare_assets 25.58, encode//sum 14.91, stage/render/render 13.58, render/graph/ms 11.43 |
| warm-up | 4.97 | 40.8 | pipelines/compile_ms 411, pipelines/front_ms 397, main/busy/ms 51.97, render/thread/ms 37.70, stage/main/update 37.56, render/busy/ms 36.35, stream/drive/ms 29.31, encode//sum 15.78, pipelines/driver_ms 13.59, stage/render/render 13.00, stage/render/prepare_assets 12.48, render/graph/ms 11.13 |
| warm-up | 5.14 | 54.0 | pipelines/compile_ms 850, pipelines/front_ms 824, render/thread/ms 54.76, render/busy/ms 52.39, main/busy/ms 43.95, stage/render/prepare_assets 26.57, pipelines/driver_ms 26.38, stage/main/update 26.05, stage/render/render 18.90, encode//sum 17.59, render/graph/ms 16.10, stage/render/graph_submit 13.16 |
| warm-up | 5.17 | 29.1 | pipelines/compile_ms 1102, pipelines/front_ms 1067, render/thread/ms 59.33, render/busy/ms 57.11, main/busy/ms 43.69, pipelines/driver_ms 35.47, stage/render/prepare_assets 32.27, stage/main/update 22.67, encode//sum 16.62, stage/render/render 16.53, stage/main/post_update 15.16, render/graph/ms 13.18 |
| warm-up | 5.20 | 34.0 | pipelines/compile_ms 307, pipelines/front_ms 295, render/thread/ms 50.16, render/busy/ms 48.06, main/busy/ms 39.13, stage/render/prepare_assets 21.60, stage/main/update 17.44, stage/render/render 17.32, stage/main/post_update 15.12, encode//sum 14.86, render/graph/ms 14.10, pipelines/driver_ms 12.58 |
| warm-up | 5.24 | 36.8 | pipelines/compile_ms 289, pipelines/front_ms 275, render/thread/ms 57.25, render/busy/ms 55.54, stage/render/prepare_assets 32.75, main/busy/ms 31.84, stage/render/render 14.83, stage/main/post_update 14.08, encode//sum 14.06, pipelines/driver_ms 13.34, render/graph/ms 12.62, stage/main/update 11.42 |
| warm-up | 5.34 | 28.7 | pipelines/compile_ms 588, pipelines/front_ms 572, render/thread/ms 44.25, render/busy/ms 42.88, main/busy/ms 26.63, stage/render/prepare_assets 26.58, pipelines/driver_ms 16.17, encode//sum 16.14, stage/main/post_update 12.17, stage/render/render 10.61, stage/main/update 9.07, render/graph/ms 8.71 |
| warm-up | 5.50 | 29.7 | pipelines/compile_ms 148, pipelines/front_ms 145, render/thread/ms 39.97, main/busy/ms 38.68, render/busy/ms 38.30, stage/main/update 21.42, encode//sum 19.56, stage/render/render 16.80, render/graph/ms 14.38, stage/main/post_update 13.41, stage/render/prepare_assets 10.98, stage/render/graph_submit 10.64 |
| warm-up | 5.93 | 32.0 | render/thread/ms 35.48, render/busy/ms 34.45, main/busy/ms 25.14, stage/render/prepare_assets 15.32, encode//sum 14.31, stage/main/update 10.86, stage/render/render 10.35, render/graph/ms 8.91, stage/main/post_update 8.87, stage/render/graph_submit 6.81, encode/finish_wall 3.84, stream/drive/ms 3.43 |
| warm-up | 6.13 | 30.1 | main/busy/ms 42.29, render/thread/ms 41.53, render/busy/ms 39.94, stage/main/update 21.39, encode//sum 21.36, stage/render/render 15.50, stage/main/post_update 14.15, render/graph/ms 13.06, stage/render/graph_submit 9.56, stream/drive/ms 6.23, stage/render/prepare_assets 6.07, encode/submit 5.55 |
| warm-up | 6.19 | 32.8 | pipelines/compile_ms 77.20, pipelines/front_ms 72.88, render/thread/ms 42.47, render/busy/ms 41.43, main/busy/ms 31.73, stage/render/prepare_assets 22.10, encode//sum 17.42, stage/main/update 14.31, stage/main/post_update 10.74, stage/render/render 9.50, render/graph/ms 7.93, stage/render/graph_submit 5.53 |
| warm-up | 6.26 | 29.3 | render/thread/ms 37.74, main/busy/ms 37.48, render/busy/ms 36.61, stage/main/update 17.60, encode//sum 16.01, stage/render/prepare_assets 15.35, stage/main/post_update 13.99, stage/render/render 10.00, render/graph/ms 8.44, stage/render/graph_submit 5.77, stream/drive/ms 3.72, encode/finish_wall 3.51 |
| warm-up | 6.43 | 36.3 | render/thread/ms 53.70, render/busy/ms 52.69, main/busy/ms 35.90, stage/render/prepare_assets 33.54, stage/main/update 23.34, encode//sum 20.80, stream/drive/ms 16.55, stage/render/render 11.91, render/graph/ms 10.41, stage/main/post_update 8.89, stage/render/graph_submit 8.11, encode/finish_wall 4.26 |
| measure | 0.52 | 62.7 | render/thread/ms 70.86, render/busy/ms 69.60, stage/render/render 64.63, render/graph/ms 62.99, stage/render/graph_submit 60.88, encode/submit 56.89, encode//sum 18.18, main/busy/ms 17.73, stage/main/post_update 11.21, stage/main/update 4.27, encode/finish_wall 4.06, stage/render/camera_driver 1.98 |
| measure | 18.19 | 30.7 | main/busy/ms 38.64, stage/main/update 24.62, encode//sum 23.35, render/thread/ms 21.77, render/busy/ms 20.61, stream/drive/ms 20.25, stage/render/render 13.67, render/graph/ms 12.12, stage/main/post_update 11.10, stage/render/graph_submit 9.60, encode/submit 5.16, encode/finish_wall 4.54 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_noslab\balmora_walk@2560x1440_off=meshslabs-3 --bench-baseline none --off meshslabs
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 0%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **77%** (min 48%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.3 W · 72 °C (max 77) · VRAM 5259 MiB · throttled 100% of samples
