# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5605 frames over 65.6 s · **median 11.44 ms (87 fps)** · mean 11.71 · p95 13.99 · p99 16.21 (1% low 62 fps) · worst 30.84 ms · 38 spikes over 1.5× median · 27.4 stutters/min (work 29.2/min)
- warm-up: 323 frames over 9.1 s, worst 2515.4 ms
- hitches (> 28.6 ms): **2** measured, 42 in warm-up
- timed GPU passes: 7.42 ms of a 11.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.44 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 30.84 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.059 | 2.074 | 2.109 | 0.99 | 0.173 |
| main_opaque_pass_3d | 1.629 | 1.641 | 2.034 | 0.99 | 0.064 |
| early prepass | 0.541 | 0.545 | 0.775 | 0.99 | 0.080 |
| bloom | 0.318 | 0.320 | 0.326 | 0.99 | 0.063 |
| atmosphere_luts | 0.312 | 0.314 | 0.372 | 0.99 | 0.003 |
| water_velocity | 0.280 | 0.282 | 0.353 | 0.99 | 0.002 |
| water_sspr | 0.204 | 0.205 | 0.206 | 0.99 | 0.002 |
| ssao | 0.190 | 0.192 | 0.195 | 0.99 | 0.007 |
| uniform_allocation | 0.179 | 0.180 | 0.455 | 0.99 | 0.025 |
| per_view_shadow_pass_cascade1 | 0.149 | 0.584 | 0.655 | 0.26 | 0.038 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.147 | 0.99 | 0.001 |
| clouds_trace | 0.141 | 0.142 | 0.236 | 0.99 | 0.001 |
| auxiliary_views | 0.138 | 0.139 | 0.538 | 0.99 | 0.127 |
| water_fft | 0.132 | 0.133 | 0.139 | 0.99 | 0.003 |
| upscaling | 0.117 | 0.118 | 0.127 | 0.99 | 0.004 |
| bin_unpacking | 0.108 | 0.109 | 0.132 | 0.99 | 0.018 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 0.99 | 0.002 |
| tonemapping | 0.086 | 0.086 | 0.090 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.083 | 0.084 | 0.160 | 0.99 | 0.008 |
| clustering | 0.064 | 0.065 | 0.068 | 0.99 | 0.019 |
| render_sky | 0.052 | 0.052 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.422 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.042 | 0.057 | 0.057 | 0.75 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.041 | 0.053 | 0.99 | 0.004 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.224 | 0.234 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.028 | 0.99 | 0.159 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.004 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_downsampling_second_pass | 0.002 | 0.018 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.041 | 0.050 | 0.01 | 0.013 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.046 | 0.053 | 0.00 | 0.011 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.997 | 16.274 | 58.91 | 61637.3 |
| render/thread/ms | 10.416 | 12.579 | 25.05 | 58380.7 |
| render/busy/ms | 9.852 | 11.996 | 24.44 | 55223.2 |
| main/busy/ms | 8.853 | 10.930 | 29.04 | 49620.5 |
| stage/render/render | 6.529 | 8.005 | 15.90 | 36594.1 |
| render/graph/ms | 5.752 | 7.156 | 14.87 | 32239.9 |
| stage/main/post_update | 5.448 | 7.183 | 9.93 | 30536.4 |
| stage/render/graph_submit | 4.554 | 5.640 | 10.98 | 25524.8 |
| encode/finish_wall | 2.349 | 3.009 | 6.78 | 13167.1 |
| encode/submit | 2.261 | 2.952 | 6.21 | 12670.9 |
| stage/main/update | 2.153 | 2.598 | 23.43 | 12065.8 |
| stage/render/camera_driver | 1.143 | 1.535 | 4.11 | 6404.2 |
| stage/render/prepare_write_phase_buffers | 0.760 | 1.057 | 2.34 | 4261.8 |
| stage/render/prepare_assets | 0.560 | 0.721 | 9.85 | 3138.7 |
| bigspace/total/ms | 0.538 | 1.369 | 3.45 | 3014.1 |
| render/present/ms | 0.474 | 0.569 | 2.48 | 2657.2 |
| stage/render/prepare_resources | 0.453 | 0.551 | 1.85 | 2540.7 |
| stage/render/prepare_meshes | 0.451 | 0.613 | 2.44 | 2528.1 |
| stage/main/first | 0.399 | 0.523 | 5.26 | 2235.0 |
| stage/main/pre_update | 0.330 | 0.490 | 1.57 | 1847.0 |
| terrain/update/ms | 0.309 | 0.408 | 2.33 | 1731.2 |
| stage/render/specialize | 0.300 | 0.432 | 4.46 | 1680.2 |
| terrain/select/ms | 0.275 | 0.368 | 0.82 | 1543.0 |
| controller/feed/ms | 0.271 | 0.344 | 0.80 | 1518.7 |
| stage/render/prepare_bind_groups | 0.267 | 0.471 | 2.93 | 1496.4 |
| stream/drive/ms | 0.242 | 0.322 | 21.53 | 1358.6 |
| stage/render/queue | 0.231 | 0.394 | 4.17 | 1297.5 |
| stage/render/prepare_batch_phases | 0.228 | 0.311 | 0.63 | 1277.8 |
| stage/main/fixed_main_loop | 0.227 | 0.369 | 1.06 | 1274.4 |
| bigspace/low_precision/ms | 0.222 | 0.661 | 3.20 | 1244.1 |
| far/select/ms | 0.186 | 0.254 | 0.73 | 1044.9 |
| stream/tick/ms | 0.186 | 0.251 | 0.64 | 1040.7 |
| anim/evaluate/ms | 0.185 | 0.284 | 2.46 | 1038.0 |
| stage/render/create_views | 0.182 | 0.237 | 0.88 | 1020.2 |
| stage/render/prepare_flush | 0.177 | 0.363 | 1.15 | 994.4 |
| stage/render/extract_commands | 0.175 | 0.206 | 0.59 | 978.3 |
| bigspace/high_precision/ms | 0.164 | 0.650 | 2.85 | 921.5 |
| anim/evaluate_cpu/ms | 0.144 | 0.187 | 0.49 | 805.7 |
| controller/move/ms | 0.132 | 0.264 | 0.53 | 737.4 |
| stage/render/prepare_views | 0.129 | 0.161 | 0.42 | 724.1 |
| stage/main/last | 0.121 | 0.194 | 1.86 | 676.9 |
| stage/render/phase_sort | 0.108 | 0.141 | 0.40 | 604.0 |
| render/acquire/ms | 0.089 | 0.114 | 0.38 | 500.2 |
| stage/render/cleanup | 0.075 | 0.193 | 1.14 | 419.4 |
| physics/step/ms | 0.051 | 0.086 | 0.50 | 284.9 |
| pipelines/compile_ms | 0.050 | 0.000 | 114.00 | 280.5 |
| pipelines/front_ms | 0.048 | 0.000 | 110.49 | 267.5 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.072 | 0.30 | 262.2 |
| physics/insert/ms | 0.035 | 0.384 | 0.93 | 198.6 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.80 | 169.4 |
| weather/map/ms | 0.025 | 0.033 | 0.24 | 139.8 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.47 | 83.7 |
| anim/apply/ms | 0.011 | 0.020 | 0.20 | 62.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.35 | 60.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.10 | 59.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.08 | 57.7 |
| script/update/ms | 0.007 | 0.008 | 0.26 | 37.7 |
| stream/despawn/ms | 0.005 | 0.000 | 0.94 | 28.3 |
| script/input/ms | 0.005 | 0.006 | 0.25 | 27.7 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 26.1 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 25.1 |
| controller/write/ms | 0.003 | 0.005 | 0.13 | 19.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.74 | 13.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.43 | 13.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.0 |
| stage/render/prepare | 0.001 | 0.002 | 0.22 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 27 | 42 | 13 |
| crowd/c0/joints/count | count | 1291 | 2218 | 583 |
| crowd/c0/meshes/count | count | 353 | 633 | 162 |
| crowd/c1/count | count | 145 | 158 | 148 |
| crowd/c1/joints/count | count | 5497 | 6045 | 5613 |
| crowd/c1/meshes/count | count | 1150 | 1313 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 237 |
| crowd/joints_extracted/count | count | 4621 | 10.1 k | 6494 |
| crowd/joints_uploaded/count | count | 4608 | 6835 | 6494 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.92 | 3.77 | 1.77 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.47 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.35 | 0.01 |
| encode/finish_critical | ms | 1.92 | 3.77 | 1.77 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.80 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.72 | 29.04 | 9.23 |
| gpu/timed/ms | ms | 8.66 | 12.15 | 9.23 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 249 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3900 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3900 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 4099 | 4330 | 4330 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 4099 | 4330 | 4330 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 199 | 5 |
| phases/shadow/bins | count | 10.8 k | 12.7 k | 8218 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 29.0 k | 32.5 k | 27.4 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 195 | 235 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 298 | 299 | 299 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 20 | 563 | 6 |
| preprocess/bind_groups_created | count | 20 | 563 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 225 | 125 |
| process/cpu_usage | % | 25.45 | 37.02 | 26.32 |
| process/mem_usage | GiB | 5.44 | 5.60 | 5.60 |
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
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.56 | 49.45 | 28.16 |
| system/mem_usage | % | 50.52 | 51.19 | 51.17 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 70 | 250 | 55 |
| views/all/bins | count | 19.4 k | 21.3 k | 17.8 k |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 56.9 k | 61.1 k | 57.0 k |
| views/all/pipelines | count | 242 | 282 | 66 |
| views/all/pipelines_distinct | count | 72 | 76 | 61 |
| views/all/work_items | count | 41.4 k | 58.3 k | 42.2 k |
| views/cascade/batch_sets | count | 11 | 32 | 5 |
| views/cascade/bins | count | 8064 | 8780 | 8218 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 26.7 k | 28.3 k | 27.4 k |
| views/cascade/pipelines | count | 25 | 34 | 20 |
| views/cascade/work_items | count | 11.8 k | 27.9 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 188 | 0 |
| views/lamp_face/bins | count | 3204 | 4181 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3454 | 4475 | 0 |
| views/lamp_face/pipelines | count | 171 | 205 | 0 |
| views/lamp_face/work_items | count | 0 | 4095 | 0 |
| views/main/batch_sets | count | 51 | 55 | 50 |
| views/main/bins | count | 8670 | 9602 | 9602 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 48 | 51 | 46 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 55.89 | 290 | 313.3 k |
| anim/events/count | count | 0.09 | 3 | 517 |
| assets/Image/added | count | 0.11 | 40 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.86 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1485 | 4675 |
| encode//count | count | 38.81 | 272 | 217.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5605 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5605 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5605 |
| physics/inserts | count | 0.40 | 87 | 2269 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 161 | 2335 |
| physics/steps | count | 0.70 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6040 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1324 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.6 ms; slowest 44, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2515.4 | main/busy/ms 2490, stage/main/post_update 2387, stage/main/update 96.56, surface/advance_cpu_ms 68.35, stage/main/pre_update 2.61, stage/main/last 1.31, surface/cell_upload_ms 0.71, stage/main/fixed_main_loop 0.57, stage/main/first 0.56, stream/drive/ms 0.55, stream/tick/ms 0.14, weather/map/ms 0.13 |
| warm-up | 3.25 | 734.2 | main/busy/ms 2635, stage/main/post_update 2417, render/thread/ms 706, render/busy/ms 544, stage/render/prepare_assets 507, stage/main/update 205, stage/render/prepare_views 158, render/acquire/ms 158, surface/advance_cpu_ms 68.35, stage/render/prepare_resources 21.60, stage/render/render 13.00, encode//sum 10.10 |
| warm-up | 3.33 | 80.4 | render/thread/ms 771, render/busy/ms 607, stage/render/prepare_assets 551, main/busy/ms 171, stage/render/prepare_views 159, render/acquire/ms 158, stage/main/update 116, stage/main/post_update 44.11, stage/render/render 25.58, stage/render/prepare_resources 24.72, pipelines/compile_ms 19.98, terrain/update/ms 19.69 |
| warm-up | 3.47 | 113.4 | pipelines/wait_ms 2291, pipelines/compile_ms 854, pipelines/front_ms 805, render/thread/ms 135, render/busy/ms 128, main/busy/ms 60.07, stage/render/prepare_assets 53.81, pipelines/driver_ms 48.61, stage/main/post_update 31.84, stage/render/render 23.02, stage/render/prepare_write_phase_buffers 22.40, stage/render/prepare_resources 18.99 |
| warm-up | 3.54 | 65.5 | pipelines/wait_ms 5430, pipelines/compile_ms 1433, pipelines/front_ms 1368, render/thread/ms 174, render/busy/ms 171, main/busy/ms 97.30, pipelines/driver_ms 65.30, stage/render/prepare_assets 58.09, stage/main/post_update 42.13, stage/main/update 34.89, stage/render/prepare_resources 34.14, stage/render/prepare_write_phase_buffers 21.96 |
| warm-up | 3.59 | 55.1 | pipelines/wait_ms 3589, pipelines/compile_ms 1560, pipelines/front_ms 1489, main/busy/ms 140, render/thread/ms 105, render/busy/ms 101, stage/main/update 86.29, pipelines/driver_ms 71.28, stage/render/prepare_assets 35.89, stage/main/post_update 31.76, stage/render/prepare_resources 18.20, bigspace/total/ms 18.12 |
| warm-up | 3.68 | 93.0 | pipelines/compile_ms 1976, pipelines/front_ms 1893, pipelines/wait_ms 672, main/busy/ms 96.40, render/thread/ms 94.10, render/busy/ms 91.99, pipelines/driver_ms 82.96, stage/render/prepare_assets 78.28, stage/main/update 69.40, stage/main/post_update 15.39, stage/render/render 8.60, stage/main/fixed_main_loop 8.45 |
| warm-up | 3.86 | 61.3 | pipelines/compile_ms 235, pipelines/front_ms 228, render/thread/ms 50.93, render/busy/ms 49.34, main/busy/ms 48.79, stage/main/update 38.22, stream/drive/ms 34.11, stage/render/prepare_assets 33.32, stage/render/render 10.59, render/graph/ms 8.60, encode//sum 7.98, pipelines/driver_ms 7.34 |
| warm-up | 3.89 | 29.9 | pipelines/compile_ms 725, pipelines/front_ms 698, render/thread/ms 57.10, render/busy/ms 55.30, stage/render/prepare_assets 38.18, main/busy/ms 38.15, pipelines/driver_ms 26.75, stage/main/update 26.19, stream/drive/ms 18.92, stage/render/render 12.43, render/graph/ms 10.02, encode//sum 7.87 |
| warm-up | 3.92 | 35.7 | pipelines/compile_ms 784, pipelines/front_ms 753, render/thread/ms 52.62, render/busy/ms 50.27, main/busy/ms 45.21, pipelines/driver_ms 31.22, stage/render/prepare_assets 30.91, stage/main/update 27.48, stream/drive/ms 20.37, stage/render/render 13.92, render/graph/ms 10.91, stage/main/post_update 8.43 |
| warm-up | 3.97 | 41.5 | pipelines/compile_ms 699, pipelines/front_ms 670, render/thread/ms 61.06, render/busy/ms 58.68, stage/render/prepare_assets 39.33, main/busy/ms 31.81, pipelines/driver_ms 28.45, stage/render/render 14.12, stage/main/update 11.47, render/graph/ms 11.06, encode//sum 10.20, stage/render/graph_submit 8.57 |
| warm-up | 4.00 | 35.8 | pipelines/compile_ms 528, pipelines/front_ms 506, render/thread/ms 55.14, main/busy/ms 53.74, render/busy/ms 52.55, stage/render/prepare_assets 35.19, stage/main/update 33.66, stream/drive/ms 26.93, pipelines/driver_ms 22.44, stage/render/render 13.05, stage/main/post_update 11.22, encode//sum 9.91 |
| warm-up | 4.08 | 81.3 | pipelines/compile_ms 356, pipelines/front_ms 342, render/thread/ms 72.79, render/busy/ms 70.65, stage/render/prepare_assets 55.29, main/busy/ms 52.53, stage/main/update 33.70, stream/drive/ms 29.01, pipelines/driver_ms 14.09, stage/main/post_update 11.23, stage/render/render 9.58, encode//sum 7.00 |
| warm-up | 4.12 | 33.2 | pipelines/compile_ms 1262, pipelines/front_ms 1211, render/thread/ms 74.19, render/busy/ms 72.38, stage/render/prepare_assets 56.66, pipelines/driver_ms 51.03, main/busy/ms 35.90, stage/main/update 17.90, stage/render/render 9.69, stage/main/post_update 9.40, render/graph/ms 7.30, encode//sum 6.90 |
| warm-up | 4.18 | 59.6 | pipelines/compile_ms 1191, pipelines/front_ms 1144, render/thread/ms 60.23, render/busy/ms 58.45, main/busy/ms 55.69, pipelines/driver_ms 47.89, stage/render/prepare_assets 43.74, stage/main/update 38.79, stage/render/render 9.85, stage/main/post_update 8.83, render/graph/ms 7.51, encode//sum 7.30 |
| warm-up | 4.22 | 47.1 | pipelines/compile_ms 452, pipelines/front_ms 434, render/thread/ms 74.66, render/busy/ms 72.91, main/busy/ms 59.98, stage/render/prepare_assets 48.07, stage/main/update 40.96, pipelines/driver_ms 17.26, stage/render/render 12.23, stage/main/post_update 10.07, render/graph/ms 10.00, encode//sum 8.35 |
| warm-up | 4.27 | 52.5 | pipelines/compile_ms 284, pipelines/front_ms 272, render/thread/ms 81.11, render/busy/ms 79.22, main/busy/ms 64.95, stage/main/update 37.32, stage/render/prepare_assets 29.37, stage/render/render 14.40, stream/drive/ms 13.30, stage/main/post_update 12.90, encode//sum 12.83, render/graph/ms 11.98 |
| warm-up | 4.40 | 126.7 | pipelines/compile_ms 1631, pipelines/front_ms 1584, pipelines/wait_ms 494, render/thread/ms 150, render/busy/ms 148, main/busy/ms 80.56, stage/render/prepare_assets 69.64, pipelines/driver_ms 46.46, stage/main/post_update 32.48, stage/main/update 32.07, encode//sum 22.28, bigspace/total/ms 19.31 |
| warm-up | 4.49 | 84.5 | pipelines/compile_ms 3283, pipelines/front_ms 3138, pipelines/wait_ms 1871, render/thread/ms 184, render/busy/ms 182, pipelines/driver_ms 144, main/busy/ms 87.06, stage/render/prepare_assets 72.67, stage/main/post_update 49.42, encode//sum 38.42, stage/render/create_views 31.04, stage/render/render 23.04 |
| warm-up | 4.57 | 85.4 | pipelines/compile_ms 2118, pipelines/front_ms 2006, pipelines/wait_ms 1651, render/thread/ms 150, render/busy/ms 148, main/busy/ms 123, pipelines/driver_ms 112, stage/main/update 82.58, stream/drive/ms 65.85, encode//sum 49.31, stage/render/render 32.95, stage/render/prepare_assets 30.81 |
| warm-up | 4.64 | 66.6 | pipelines/compile_ms 1812, pipelines/front_ms 1762, pipelines/wait_ms 1049, render/thread/ms 113, render/busy/ms 110, main/busy/ms 95.75, stage/main/update 74.64, stream/drive/ms 64.20, pipelines/driver_ms 49.83, stage/render/prepare_assets 45.54, encode//sum 30.95, stage/render/render 24.98 |
| warm-up | 4.67 | 29.7 | pipelines/compile_ms 1824, pipelines/front_ms 1775, pipelines/wait_ms 1139, main/busy/ms 102, stage/main/update 79.95, render/thread/ms 67.23, render/busy/ms 65.12, pipelines/driver_ms 48.94, stream/drive/ms 48.53, stage/render/prepare_assets 33.60, stage/render/render 20.97, render/graph/ms 18.11 |
| warm-up | 4.78 | 108.9 | pipelines/compile_ms 1288, pipelines/front_ms 1259, pipelines/wait_ms 371, main/busy/ms 97.19, stage/main/update 80.49, render/thread/ms 62.16, render/busy/ms 60.34, stream/drive/ms 48.13, stage/render/prepare_assets 29.04, pipelines/driver_ms 28.43, stage/render/render 21.87, render/graph/ms 19.51 |
| warm-up | 4.89 | 83.6 | pipelines/compile_ms 445, pipelines/front_ms 430, main/busy/ms 79.21, render/thread/ms 58.81, stage/main/update 57.67, render/busy/ms 57.18, stream/drive/ms 43.76, stage/render/prepare_assets 21.95, encode//sum 17.36, stage/render/render 16.93, pipelines/driver_ms 14.91, render/graph/ms 14.67 |
| warm-up | 4.98 | 67.0 | pipelines/compile_ms 1103, pipelines/front_ms 1069, main/busy/ms 69.33, stage/main/update 53.70, render/thread/ms 49.33, render/busy/ms 47.63, stream/drive/ms 36.23, pipelines/driver_ms 34.57, encode//sum 16.83, stage/render/prepare_assets 16.40, stage/render/render 14.08, render/graph/ms 11.68 |
| warm-up | 5.07 | 31.3 | pipelines/compile_ms 894, pipelines/front_ms 863, render/thread/ms 48.34, render/busy/ms 46.85, main/busy/ms 33.92, pipelines/driver_ms 30.86, stage/render/prepare_assets 16.82, stage/main/update 15.07, encode//sum 14.06, stage/main/post_update 13.80, stage/render/render 11.95, render/graph/ms 10.06 |
| warm-up | 5.10 | 31.5 | pipelines/compile_ms 1132, pipelines/front_ms 1096, render/thread/ms 55.92, render/busy/ms 54.27, main/busy/ms 45.26, pipelines/driver_ms 36.89, stage/main/update 20.43, stage/main/post_update 19.53, encode//sum 17.62, stage/render/prepare_assets 15.65, stage/render/render 13.81, render/graph/ms 11.59 |
| warm-up | 5.13 | 35.1 | pipelines/compile_ms 554, pipelines/front_ms 533, render/thread/ms 57.40, render/busy/ms 55.47, main/busy/ms 47.47, stage/main/update 24.85, pipelines/driver_ms 20.64, encode//sum 18.22, stage/main/post_update 17.15, stage/render/render 16.83, render/graph/ms 14.44, stage/render/prepare_assets 12.65 |
| warm-up | 5.16 | 31.3 | pipelines/compile_ms 373, pipelines/front_ms 364, render/thread/ms 51.56, render/busy/ms 49.76, main/busy/ms 40.04, stage/main/update 23.85, stage/render/prepare_assets 19.21, encode//sum 14.63, stage/render/render 13.73, render/graph/ms 11.62, stream/drive/ms 11.13, stage/main/post_update 10.67 |
| warm-up | 5.28 | 35.3 | pipelines/compile_ms 667, pipelines/front_ms 645, main/busy/ms 37.64, render/thread/ms 32.54, render/busy/ms 29.60, pipelines/driver_ms 22.31, stage/main/update 20.95, encode//sum 15.34, stage/render/render 11.73, stage/main/post_update 11.49, stream/drive/ms 10.11, render/graph/ms 8.18 |
| warm-up | 5.95 | 39.0 | render/thread/ms 53.37, render/busy/ms 51.51, main/busy/ms 32.34, encode//sum 21.42, stage/render/prepare_assets 16.25, stage/render/render 14.95, stage/main/post_update 12.48, render/graph/ms 12.43, stage/main/update 12.06, stage/render/graph_submit 9.38, stage/render/specialize 7.82, encode/submit 5.37 |
| warm-up | 5.99 | 35.6 | render/thread/ms 58.45, render/busy/ms 57.23, main/busy/ms 30.03, stage/render/prepare_assets 25.46, encode//sum 17.27, stage/main/update 13.77, stage/render/render 9.66, stage/render/specialize 9.03, stage/main/post_update 8.63, render/graph/ms 7.95, stage/render/graph_submit 6.08, stream/drive/ms 4.33 |
| warm-up | 6.02 | 34.1 | render/thread/ms 57.47, render/busy/ms 56.32, stage/render/prepare_assets 28.89, main/busy/ms 26.48, encode//sum 16.25, stage/main/update 11.69, stage/render/render 11.14, render/graph/ms 9.52, stage/main/post_update 9.12, stage/render/graph_submit 7.61, stage/render/specialize 5.36, encode/finish_wall 3.96 |
| warm-up | 6.07 | 34.7 | render/thread/ms 48.08, render/busy/ms 46.94, main/busy/ms 27.24, encode//sum 21.38, stage/render/prepare_assets 21.21, stage/main/update 12.34, stage/render/render 10.67, stage/main/post_update 10.24, render/graph/ms 9.20, stage/render/graph_submit 6.68, stage/render/specialize 5.60, stream/drive/ms 3.51 |
| warm-up | 6.13 | 34.0 | render/thread/ms 47.57, render/busy/ms 46.59, stage/render/prepare_assets 24.71, main/busy/ms 23.76, encode//sum 14.40, stage/main/update 11.38, stage/render/render 9.72, stage/main/post_update 8.50, render/graph/ms 8.39, stage/render/graph_submit 6.55, stage/render/specialize 3.67, stream/drive/ms 3.47 |
| warm-up | 6.17 | 30.0 | render/thread/ms 38.68, render/busy/ms 37.62, main/busy/ms 29.15, encode//sum 18.24, stage/main/update 15.61, stage/render/prepare_assets 13.80, stage/render/render 10.88, render/graph/ms 9.23, stream/drive/ms 9.05, stage/main/post_update 8.74, stage/render/graph_submit 6.60, encode/finish_wall 3.53 |
| warm-up | 6.32 | 35.5 | render/thread/ms 42.11, render/busy/ms 41.08, main/busy/ms 29.76, stage/render/prepare_assets 20.13, encode//sum 18.46, stage/main/update 11.76, stage/main/post_update 11.47, stage/render/render 10.02, render/graph/ms 8.47, stage/render/graph_submit 6.08, stream/drive/ms 3.43, encode/finish_wall 3.42 |
| warm-up | 6.35 | 28.7 | render/thread/ms 51.75, render/busy/ms 50.50, encode//sum 32.27, main/busy/ms 32.24, stage/render/render 17.49, stage/render/prepare_assets 17.25, render/graph/ms 15.73, stage/main/post_update 14.07, stage/main/update 11.80, stage/render/graph_submit 11.57, encode/submit 6.53, encode/finish_wall 5.10 |
| warm-up | 6.40 | 31.8 | main/busy/ms 44.99, render/thread/ms 41.33, render/busy/ms 39.68, stage/main/update 29.16, encode//sum 24.86, stage/render/render 15.80, stream/drive/ms 14.17, render/graph/ms 13.29, stage/main/post_update 11.41, stage/render/prepare_assets 10.97, stage/render/graph_submit 10.42, encode/finish_wall 5.43 |
| warm-up | 6.43 | 31.4 | main/busy/ms 45.79, render/thread/ms 43.94, render/busy/ms 42.32, stage/main/update 31.99, encode//sum 23.05, stream/drive/ms 17.33, stage/render/prepare_assets 15.30, stage/render/render 14.31, render/graph/ms 11.89, stage/main/post_update 9.10, stage/render/graph_submit 9.08, encode/finish_wall 5.46 |
| warm-up | 6.64 | 28.8 | render/thread/ms 41.78, render/busy/ms 40.59, main/busy/ms 24.97, stage/render/prepare_assets 19.90, encode//sum 18.22, stage/main/update 11.79, stage/render/render 11.16, stage/main/post_update 9.90, render/graph/ms 9.63, stage/render/graph_submit 7.40, encode/submit 3.82, encode/finish_wall 3.67 |
| warm-up | 6.70 | 29.7 | render/thread/ms 40.31, render/busy/ms 39.08, main/busy/ms 33.33, encode//sum 26.79, stage/main/post_update 15.30, stage/render/render 14.79, render/graph/ms 13.00, stage/main/update 12.85, stage/render/graph_submit 8.82, stage/render/specialize 5.96, encode/finish_wall 5.05, stage/render/prepare_assets 4.85 |
| measure | 18.18 | 30.8 | main/busy/ms 38.38, stage/main/update 25.21, encode//sum 23.42, render/thread/ms 23.05, render/busy/ms 21.95, stream/drive/ms 21.76, stage/render/render 14.12, render/graph/ms 12.46, stage/main/post_update 10.69, stage/render/graph_submit 9.82, encode/submit 5.10, encode/finish_wall 4.81 |
| measure | 59.05 | 29.4 | render/thread/ms 42.54, render/busy/ms 41.22, main/busy/ms 25.54, encode//sum 25.19, stage/render/prepare_assets 14.22, stage/render/render 13.16, stage/main/post_update 13.09, render/graph/ms 11.52, stage/render/graph_submit 8.70, stage/main/update 7.22, encode/finish_wall 4.61, encode/submit 4.18 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_h1\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 6%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **78%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.6 W · 72 °C (max 77) · VRAM 5266 MiB · throttled 100% of samples
