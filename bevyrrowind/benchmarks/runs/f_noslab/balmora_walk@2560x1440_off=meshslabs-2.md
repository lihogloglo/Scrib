# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6025 frames over 65.6 s · **median 10.72 ms (93 fps)** · mean 10.89 · p95 13.02 · p99 15.29 (1% low 65 fps) · worst 28.09 ms · 37 spikes over 1.5× median · 27.4 stutters/min (work 22.9/min)
- warm-up: 324 frames over 9.0 s, worst 2556.5 ms
- hitches (> 26.8 ms): **2** measured, 48 in warm-up
- timed GPU passes: 6.83 ms of a 10.89 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.72 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 28.09 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.930 | 2.069 | 2.101 | 0.93 | 0.169 |
| main_opaque_pass_3d | 1.515 | 1.624 | 2.028 | 0.93 | 0.054 |
| early prepass | 0.507 | 0.544 | 0.929 | 0.93 | 0.077 |
| atmosphere_luts | 0.313 | 0.335 | 0.627 | 0.93 | 0.003 |
| bloom | 0.298 | 0.319 | 0.325 | 0.93 | 0.061 |
| water_velocity | 0.266 | 0.286 | 0.353 | 0.93 | 0.001 |
| water_sspr | 0.192 | 0.206 | 0.205 | 0.93 | 0.002 |
| ssao | 0.187 | 0.200 | 0.209 | 0.93 | 0.006 |
| atmos_froxel_inject | 0.137 | 0.146 | 0.147 | 0.93 | 0.001 |
| clouds_trace | 0.132 | 0.142 | 0.236 | 0.93 | 0.001 |
| water_fft | 0.126 | 0.135 | 0.141 | 0.93 | 0.003 |
| per_view_shadow_pass_cascade1 | 0.119 | 0.582 | 0.651 | 0.20 | 0.040 |
| uniform_allocation | 0.118 | 0.127 | 0.158 | 0.93 | 0.023 |
| upscaling | 0.110 | 0.117 | 0.127 | 0.93 | 0.004 |
| clouds_composite | 0.101 | 0.108 | 0.110 | 0.93 | 0.002 |
| auxiliary_views | 0.098 | 0.105 | 0.414 | 0.93 | 0.098 |
| bin_unpacking | 0.094 | 0.101 | 0.123 | 0.93 | 0.017 |
| tonemapping | 0.080 | 0.086 | 0.090 | 0.93 | 0.003 |
| main_transmissive_pass_3d | 0.078 | 0.084 | 0.162 | 0.93 | 0.008 |
| clustering | 0.061 | 0.066 | 0.068 | 0.93 | 0.019 |
| render_sky | 0.049 | 0.052 | 0.053 | 0.93 | 0.002 |
| clouds_resolve | 0.046 | 0.049 | 0.050 | 0.93 | 0.001 |
| surface_trail | 0.037 | 0.057 | 0.057 | 0.66 | 0.002 |
| lightprobe_irradiance_map | 0.036 | 0.435 | 0.713 | 0.08 | 0.001 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.042 | 0.93 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.018 | 0.222 | 0.233 | 0.08 | 0.003 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.025 | 0.93 | 0.110 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.012 | 0.93 | 0.003 |
| per_view_shadow_pass | 0.009 | 0.010 | 0.015 | 0.93 | 0.004 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.93 | 0.002 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.08 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.024 | 0.049 | 0.01 | 0.009 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.016 | 0.017 | 0.00 | 0.014 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.553 | 11.354 | 24.57 | 57558.9 |
| encode//sum | 9.443 | 13.703 | 52.10 | 56894.8 |
| render/busy/ms | 8.956 | 10.813 | 24.01 | 53960.0 |
| main/busy/ms | 8.918 | 10.883 | 25.53 | 53730.6 |
| stage/render/render | 6.053 | 7.335 | 15.88 | 36470.7 |
| stage/main/post_update | 5.571 | 7.275 | 10.05 | 33567.9 |
| render/graph/ms | 5.271 | 6.551 | 14.83 | 31755.4 |
| stage/render/graph_submit | 4.202 | 5.276 | 11.30 | 25318.7 |
| encode/submit | 2.177 | 2.975 | 6.59 | 13117.3 |
| stage/main/update | 2.127 | 2.524 | 19.11 | 12812.9 |
| encode/finish_wall | 2.115 | 2.691 | 5.54 | 12745.7 |
| stage/render/camera_driver | 1.016 | 1.335 | 3.78 | 6121.5 |
| stage/render/prepare_write_phase_buffers | 0.702 | 0.946 | 2.33 | 4227.8 |
| bigspace/total/ms | 0.654 | 2.077 | 3.56 | 3940.0 |
| stage/render/prepare_assets | 0.546 | 0.700 | 9.80 | 3289.0 |
| render/present/ms | 0.509 | 1.224 | 2.21 | 3066.3 |
| stage/main/first | 0.397 | 0.514 | 4.37 | 2391.0 |
| stage/render/prepare_resources | 0.372 | 0.466 | 0.85 | 2243.8 |
| stage/render/prepare_meshes | 0.370 | 0.476 | 1.91 | 2231.4 |
| stage/main/pre_update | 0.323 | 0.482 | 1.18 | 1947.3 |
| terrain/update/ms | 0.301 | 0.388 | 1.72 | 1813.6 |
| bigspace/low_precision/ms | 0.271 | 1.168 | 3.18 | 1635.6 |
| stage/render/specialize | 0.270 | 0.396 | 2.66 | 1626.8 |
| terrain/select/ms | 0.268 | 0.346 | 0.69 | 1615.7 |
| controller/feed/ms | 0.263 | 0.329 | 0.64 | 1584.1 |
| stage/render/prepare_bind_groups | 0.245 | 0.393 | 2.75 | 1476.2 |
| stream/drive/ms | 0.241 | 0.315 | 17.13 | 1454.3 |
| stage/render/queue | 0.235 | 0.375 | 3.64 | 1412.9 |
| bigspace/high_precision/ms | 0.227 | 1.179 | 3.24 | 1365.4 |
| anim/evaluate/ms | 0.221 | 0.527 | 2.54 | 1332.4 |
| stage/main/fixed_main_loop | 0.209 | 0.355 | 1.10 | 1261.2 |
| stream/tick/ms | 0.187 | 0.253 | 0.58 | 1128.0 |
| far/select/ms | 0.183 | 0.244 | 0.53 | 1103.3 |
| stage/render/prepare_flush | 0.167 | 0.346 | 0.81 | 1004.8 |
| stage/render/create_views | 0.166 | 0.216 | 0.84 | 1000.4 |
| stage/render/extract_commands | 0.163 | 0.193 | 0.54 | 982.8 |
| stage/render/prepare_batch_phases | 0.158 | 0.218 | 0.51 | 951.3 |
| anim/evaluate_cpu/ms | 0.141 | 0.179 | 0.50 | 849.2 |
| controller/move/ms | 0.131 | 0.223 | 0.61 | 786.6 |
| stage/render/prepare_views | 0.129 | 0.165 | 0.44 | 777.9 |
| stage/main/last | 0.117 | 0.183 | 1.61 | 704.7 |
| render/acquire/ms | 0.088 | 0.115 | 0.34 | 532.6 |
| stage/render/phase_sort | 0.088 | 0.135 | 0.42 | 528.4 |
| pipelines/compile_ms | 0.048 | 0.000 | 116.98 | 286.2 |
| physics/step/ms | 0.046 | 0.082 | 0.55 | 277.3 |
| pipelines/front_ms | 0.046 | 0.000 | 112.57 | 274.4 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.070 | 0.29 | 272.3 |
| stage/render/cleanup | 0.032 | 0.100 | 0.55 | 191.2 |
| physics/insert/ms | 0.031 | 0.350 | 0.89 | 187.9 |
| encode/resolve_diagnostics/sum | 0.029 | 0.040 | 0.41 | 177.4 |
| weather/map/ms | 0.025 | 0.033 | 0.28 | 149.3 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.12 | 87.1 |
| anim/apply/ms | 0.011 | 0.019 | 0.14 | 64.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.44 | 64.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.16 | 62.3 |
| controller/avoid/ms | 0.010 | 0.013 | 0.04 | 61.3 |
| script/update/ms | 0.006 | 0.008 | 0.04 | 38.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.00 | 28.2 |
| script/input/ms | 0.005 | 0.006 | 0.11 | 27.9 |
| script/ui/ms | 0.005 | 0.006 | 0.25 | 27.6 |
| stream/spawn/ms | 0.004 | 0.000 | 1.07 | 26.1 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 20.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.6 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.42 | 11.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 15 |
| crowd/c0/joints/count | count | 1306 | 2262 | 696 |
| crowd/c0/meshes/count | count | 353 | 646 | 201 |
| crowd/c1/count | count | 145 | 157 | 146 |
| crowd/c1/joints/count | count | 5531 | 6056 | 5500 |
| crowd/c1/meshes/count | count | 1160 | 1300 | 1140 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4603 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.78 | 3.33 | 1.87 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.12 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.44 | 0.02 |
| encode/finish_critical | ms | 1.78 | 3.33 | 1.87 |
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
| frame/work/ms | ms | 9.33 | 25.53 | 9.11 |
| gpu/timed/ms | ms | 8.51 | 12.66 | 9.11 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 193 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 827 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 193 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 827 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1289 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3454 | 5097 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1289 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3454 | 5097 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 205 | 4 |
| phases/shadow/bins | count | 4372 | 5406 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.2 k | 12.5 k | 5438 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 190 | 226 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2657 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 183 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 567 | 6 |
| preprocess/bind_groups_created | count | 12 | 567 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 219 | 123 |
| process/cpu_usage | % | 26.92 | 42.47 | 21.74 |
| process/mem_usage | GiB | 5.39 | 5.56 | 5.56 |
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
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.34 | 53.22 | 28.72 |
| system/mem_usage | % | 54.01 | 54.41 | 54.36 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 63 | 252 | 54 |
| views/all/bins | count | 6979 | 8205 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 24.9 k | 13.1 k |
| views/all/pipelines | count | 235 | 273 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 22.4 k | 8159 |
| views/cascade/batch_sets | count | 10 | 33 | 4 |
| views/cascade/bins | count | 2226 | 2876 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7673 | 9643 | 5438 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 523 | 9543 | 239 |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 2133 | 2718 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2385 | 3014 | 0 |
| views/lamp_face/pipelines | count | 166 | 196 | 0 |
| views/lamp_face/work_items | count | 0 | 2949 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2870 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8568 | 13.0 k | 7612 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8724 | 13.2 k | 7920 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.9 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 48.15 | 222 | 290.1 k |
| anim/events/count | count | 0.08 | 3 | 471 |
| assets/Image/added | count | 0.10 | 38 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.59 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1413 | 4675 |
| encode//count | count | 37.13 | 272 | 223.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6025 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6025 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6025 |
| physics/inserts | count | 0.37 | 87 | 2246 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 161 | 2335 |
| physics/steps | count | 0.65 | 2 | 3938 |
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
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5619 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1232 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.8 ms; slowest 50, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2556.5 | main/busy/ms 2528, stage/main/post_update 2421, stage/main/update 101, surface/advance_cpu_ms 67.33, stage/main/pre_update 2.49, stage/main/last 1.16, surface/cell_upload_ms 0.71, stage/main/first 0.71, stream/drive/ms 0.53, stage/main/fixed_main_loop 0.38, stream/tick/ms 0.19, weather/map/ms 0.14 |
| warm-up | 3.30 | 742.8 | main/busy/ms 2678, stage/main/post_update 2453, render/thread/ms 714, render/busy/ms 553, stage/render/prepare_assets 518, stage/main/update 213, stage/render/prepare_views 158, render/acquire/ms 157, surface/advance_cpu_ms 67.33, stage/render/prepare_resources 21.40, stage/render/render 12.09, encode//sum 10.52 |
| warm-up | 3.38 | 82.7 | render/thread/ms 780, render/busy/ms 618, stage/render/prepare_assets 564, main/busy/ms 174, stage/render/prepare_views 158, render/acquire/ms 157, pipelines/compile_ms 136, pipelines/front_ms 125, stage/main/update 117, stage/main/post_update 44.57, stage/render/prepare_resources 26.39, terrain/update/ms 24.26 |
| warm-up | 3.43 | 47.8 | pipelines/compile_ms 290, pipelines/front_ms 270, render/thread/ms 112, render/busy/ms 98.68, pipelines/wait_ms 94.40, main/busy/ms 71.14, stage/main/post_update 52.77, stage/render/render 50.27, stage/render/prepare_assets 46.66, render/graph/ms 37.31, terrain/update/ms 30.09, stage/render/graph_submit 22.38 |
| warm-up | 3.60 | 146.5 | pipelines/wait_ms 6065, pipelines/compile_ms 2061, pipelines/front_ms 1948, main/busy/ms 164, render/thread/ms 113, pipelines/driver_ms 113, stage/main/update 112, render/busy/ms 111, stage/render/prepare_resources 40.78, stage/main/post_update 34.35, stage/render/prepare_assets 26.18, terrain/update/ms 18.70 |
| warm-up | 3.66 | 56.7 | pipelines/wait_ms 5480, pipelines/compile_ms 2500, pipelines/front_ms 2368, render/thread/ms 144, render/busy/ms 142, pipelines/driver_ms 132, main/busy/ms 109, stage/main/update 79.94, stage/render/prepare_assets 64.03, stage/render/prepare_resources 40.10, stage/main/post_update 23.06, stage/render/prepare_flush 14.76 |
| warm-up | 3.69 | 30.9 | pipelines/compile_ms 693, pipelines/front_ms 653, pipelines/wait_ms 166, render/thread/ms 78.67, render/busy/ms 76.39, stage/render/prepare_assets 59.96, pipelines/driver_ms 39.17, main/busy/ms 37.81, stage/main/post_update 20.55, terrain/update/ms 13.18, stage/main/update 11.07, stage/render/render 10.51 |
| warm-up | 3.72 | 36.1 | pipelines/compile_ms 333, pipelines/front_ms 316, pipelines/wait_ms 101, render/thread/ms 57.81, render/busy/ms 55.75, stage/render/prepare_assets 38.56, main/busy/ms 28.99, pipelines/driver_ms 16.99, stage/main/post_update 13.65, terrain/update/ms 11.65, stage/render/render 9.21, stage/main/update 8.75 |
| warm-up | 3.76 | 37.9 | pipelines/compile_ms 742, pipelines/front_ms 701, pipelines/wait_ms 145, render/thread/ms 68.83, render/busy/ms 66.41, main/busy/ms 49.49, pipelines/driver_ms 41.15, stage/render/prepare_assets 26.74, encode//sum 19.95, stage/render/prepare_write_phase_buffers 16.99, stage/main/post_update 16.71, stage/render/render 12.51 |
| warm-up | 3.79 | 29.5 | pipelines/compile_ms 681, pipelines/front_ms 647, render/thread/ms 62.75, pipelines/wait_ms 61.94, render/busy/ms 60.40, main/busy/ms 47.80, pipelines/driver_ms 34.46, stage/render/prepare_assets 21.72, encode//sum 19.10, stage/render/prepare_write_phase_buffers 16.97, stage/main/post_update 16.91, stage/render/render 13.00 |
| warm-up | 3.82 | 27.6 | pipelines/compile_ms 467, pipelines/front_ms 448, render/thread/ms 52.41, render/busy/ms 50.20, encode//sum 37.15, stage/render/prepare_assets 23.42, main/busy/ms 23.09, pipelines/driver_ms 18.88, pipelines/wait_ms 18.43, stage/render/render 13.82, stage/main/post_update 11.50, render/graph/ms 11.46 |
| warm-up | 3.92 | 77.8 | pipelines/compile_ms 1437, pipelines/front_ms 1361, pipelines/wait_ms 183, pipelines/driver_ms 75.98, main/busy/ms 62.67, render/thread/ms 62.12, render/busy/ms 60.66, stage/main/update 48.08, stream/drive/ms 42.86, stage/render/prepare_assets 39.92, stage/render/render 9.87, stage/main/post_update 9.70 |
| warm-up | 3.95 | 31.6 | pipelines/compile_ms 1308, pipelines/front_ms 1229, pipelines/wait_ms 185, pipelines/driver_ms 78.15, render/thread/ms 63.38, render/busy/ms 58.27, stage/render/prepare_assets 40.01, main/busy/ms 37.00, stage/main/update 20.08, stage/render/render 16.32, stream/drive/ms 11.72, render/graph/ms 10.88 |
| warm-up | 4.01 | 26.9 | pipelines/compile_ms 469, pipelines/front_ms 455, main/busy/ms 60.68, render/thread/ms 43.96, render/busy/ms 41.97, stage/main/update 36.59, stream/drive/ms 26.17, stage/render/prepare_assets 24.17, stage/main/post_update 17.27, pipelines/driver_ms 13.39, stage/render/render 11.12, render/graph/ms 8.45 |
| warm-up | 4.11 | 100.4 | pipelines/compile_ms 455, pipelines/front_ms 439, render/thread/ms 78.46, render/busy/ms 75.93, main/busy/ms 74.54, stage/render/prepare_assets 50.62, stage/main/update 42.80, stream/drive/ms 26.98, stage/main/post_update 19.69, stage/render/render 15.44, pipelines/driver_ms 15.24, render/graph/ms 11.79 |
| warm-up | 4.16 | 54.5 | pipelines/compile_ms 1126, pipelines/front_ms 1083, render/thread/ms 92.49, render/busy/ms 90.00, main/busy/ms 62.76, stage/render/prepare_assets 61.22, pipelines/driver_ms 43.03, stage/main/update 38.77, stage/render/render 15.06, stage/main/post_update 12.45, render/graph/ms 11.66, stage/render/graph_submit 8.20 |
| warm-up | 4.30 | 132.7 | pipelines/compile_ms 1996, pipelines/front_ms 1934, render/thread/ms 130, render/busy/ms 128, stage/render/prepare_assets 70.89, pipelines/driver_ms 61.56, main/busy/ms 55.89, pipelines/wait_ms 50.79, stage/main/update 33.79, stage/render/prepare_write_phase_buffers 29.82, stage/render/render 14.36, stage/main/post_update 12.87 |
| warm-up | 4.37 | 78.4 | pipelines/compile_ms 2398, pipelines/front_ms 2274, pipelines/wait_ms 274, render/thread/ms 169, render/busy/ms 167, encode//sum 155, pipelines/driver_ms 124, main/busy/ms 81.24, stage/render/prepare_assets 62.48, stage/render/render 41.36, render/graph/ms 39.03, stage/render/graph_submit 33.78 |
| warm-up | 4.43 | 54.9 | pipelines/compile_ms 1369, pipelines/front_ms 1269, pipelines/wait_ms 386, encode//sum 159, render/thread/ms 117, render/busy/ms 115, pipelines/driver_ms 99.80, main/busy/ms 79.90, stage/render/render 43.09, render/graph/ms 40.70, stage/render/graph_submit 34.63, stage/main/update 29.26 |
| warm-up | 4.51 | 83.9 | pipelines/compile_ms 1334, pipelines/front_ms 1296, pipelines/wait_ms 252, render/thread/ms 122, render/busy/ms 120, main/busy/ms 72.55, encode//sum 68.70, stage/main/post_update 56.78, pipelines/driver_ms 38.53, stage/render/prepare_resources 29.32, stage/render/render 27.70, stage/render/prepare_write_phase_buffers 27.64 |
| warm-up | 4.54 | 31.0 | pipelines/compile_ms 1448, pipelines/front_ms 1407, pipelines/wait_ms 308, render/thread/ms 110, render/busy/ms 108, main/busy/ms 86.74, encode//sum 60.67, stage/main/post_update 58.43, pipelines/driver_ms 41.07, stage/render/prepare_resources 29.18, stage/render/prepare_write_phase_buffers 27.35, stage/render/render 27.32 |
| warm-up | 4.58 | 34.3 | pipelines/compile_ms 846, pipelines/front_ms 823, pipelines/wait_ms 276, render/thread/ms 49.88, main/busy/ms 49.58, render/busy/ms 48.02, stage/main/update 27.94, pipelines/driver_ms 23.24, stage/render/prepare_assets 14.98, stage/render/render 14.02, stage/main/post_update 12.95, render/graph/ms 11.71 |
| warm-up | 4.61 | 29.0 | pipelines/compile_ms 1123, pipelines/front_ms 1092, pipelines/wait_ms 127, main/busy/ms 83.24, stage/main/update 63.39, stream/drive/ms 50.43, render/thread/ms 44.62, render/busy/ms 42.93, pipelines/driver_ms 31.49, stage/render/prepare_assets 18.84, stage/render/render 13.56, encode//sum 13.19 |
| warm-up | 4.68 | 75.3 | pipelines/compile_ms 1641, pipelines/front_ms 1588, pipelines/wait_ms 731, main/busy/ms 100, stage/main/update 71.75, render/thread/ms 53.22, pipelines/driver_ms 53.05, stream/drive/ms 51.48, render/busy/ms 45.25, stage/render/render 23.66, stage/main/post_update 17.47, encode//sum 15.42 |
| warm-up | 4.75 | 71.3 | pipelines/compile_ms 1805, pipelines/front_ms 1742, pipelines/wait_ms 832, main/busy/ms 109, stage/main/update 83.38, render/thread/ms 77.41, render/busy/ms 69.34, pipelines/driver_ms 63.12, stream/drive/ms 43.63, stage/render/prepare_assets 28.36, stage/render/render 23.99, encode//sum 17.81 |
| warm-up | 4.88 | 130.0 | pipelines/compile_ms 1258, pipelines/front_ms 1221, pipelines/wait_ms 167, render/thread/ms 123, render/busy/ms 122, main/busy/ms 97.26, stage/main/update 76.14, stage/render/prepare_assets 64.61, stream/drive/ms 42.97, pipelines/driver_ms 37.16, stage/render/prepare_resources 24.23, encode//sum 17.47 |
| warm-up | 4.92 | 39.3 | pipelines/compile_ms 741, pipelines/front_ms 719, render/thread/ms 111, render/busy/ms 110, main/busy/ms 81.69, stage/main/update 57.17, stage/render/prepare_assets 47.64, stream/drive/ms 38.09, stage/render/render 29.09, render/graph/ms 26.78, stage/render/prepare_resources 23.93, stage/render/graph_submit 23.29 |
| warm-up | 5.00 | 73.9 | pipelines/compile_ms 140, pipelines/front_ms 130, main/busy/ms 79.02, render/thread/ms 70.75, render/busy/ms 69.07, stage/main/update 58.59, stream/drive/ms 37.64, stage/render/prepare_assets 29.42, stage/render/render 28.75, render/graph/ms 26.37, stage/render/graph_submit 22.93, encode//sum 17.87 |
| warm-up | 5.03 | 33.1 | pipelines/compile_ms 174, pipelines/front_ms 168, main/busy/ms 68.03, render/thread/ms 62.73, render/busy/ms 61.14, stage/main/update 52.16, stream/drive/ms 34.30, stage/render/prepare_assets 30.97, stage/render/render 13.55, encode//sum 13.28, render/graph/ms 11.42, stage/main/post_update 9.18 |
| warm-up | 5.07 | 39.7 | pipelines/compile_ms 812, pipelines/front_ms 785, main/busy/ms 59.22, render/thread/ms 43.25, stage/main/update 42.92, render/busy/ms 41.68, stream/drive/ms 34.68, pipelines/driver_ms 27.57, stage/render/render 13.62, encode//sum 13.48, stage/render/prepare_assets 12.77, render/graph/ms 11.59 |
| warm-up | 5.19 | 30.1 | pipelines/compile_ms 767, pipelines/front_ms 740, render/thread/ms 46.66, render/busy/ms 44.72, main/busy/ms 34.16, pipelines/driver_ms 27.09, encode//sum 18.14, stage/main/update 16.60, stage/render/render 13.91, stage/render/prepare_assets 13.70, stage/main/post_update 11.49, render/graph/ms 11.38 |
| warm-up | 5.23 | 35.5 | pipelines/compile_ms 876, pipelines/front_ms 847, render/thread/ms 52.97, render/busy/ms 51.46, main/busy/ms 40.04, pipelines/driver_ms 28.67, stage/render/prepare_assets 27.17, stage/main/update 22.82, encode//sum 14.21, stage/render/render 13.32, stage/main/post_update 11.15, render/graph/ms 11.12 |
| warm-up | 5.26 | 30.5 | pipelines/compile_ms 599, pipelines/front_ms 580, render/thread/ms 52.51, render/busy/ms 51.06, main/busy/ms 36.38, stage/render/prepare_assets 28.35, stage/main/update 20.06, pipelines/driver_ms 18.99, stage/render/render 13.11, encode//sum 11.43, render/graph/ms 11.13, stage/main/post_update 10.47 |
| warm-up | 5.29 | 30.9 | pipelines/compile_ms 529, pipelines/front_ms 511, render/thread/ms 53.78, render/busy/ms 52.15, main/busy/ms 42.91, stage/main/update 18.61, stage/main/post_update 18.60, pipelines/driver_ms 18.19, stage/render/render 17.69, render/graph/ms 15.60, stage/render/prepare_assets 14.42, encode//sum 13.29 |
| warm-up | 5.35 | 56.0 | pipelines/compile_ms 95.09, pipelines/front_ms 90.81, render/thread/ms 81.83, render/busy/ms 79.70, main/busy/ms 42.17, stage/render/render 23.91, stage/render/prepare_assets 23.63, render/graph/ms 21.16, stage/main/update 20.00, stage/main/post_update 16.60, encode//sum 14.18, stage/render/prepare_resources 12.59 |
| warm-up | 5.38 | 35.5 | render/thread/ms 85.88, render/busy/ms 83.82, pipelines/compile_ms 82.59, pipelines/front_ms 78.70, main/busy/ms 36.25, stage/render/prepare_assets 31.38, stage/render/render 21.59, stage/main/update 19.13, render/graph/ms 18.81, encode//sum 18.24, stage/render/graph_submit 16.07, stage/render/prepare_resources 14.56 |
| warm-up | 5.43 | 46.0 | pipelines/compile_ms 1016, pipelines/front_ms 989, render/thread/ms 69.39, render/busy/ms 67.73, main/busy/ms 37.31, stage/render/prepare_assets 35.73, pipelines/driver_ms 27.50, encode//sum 17.29, stage/render/render 16.08, stage/main/update 15.62, stage/main/post_update 15.22, render/graph/ms 13.83 |
| warm-up | 5.46 | 29.1 | pipelines/compile_ms 1973, pipelines/front_ms 1911, render/thread/ms 63.50, pipelines/driver_ms 61.71, render/busy/ms 61.13, main/busy/ms 43.58, stage/render/prepare_assets 30.89, stage/main/post_update 19.55, stage/main/update 18.26, stage/render/render 14.29, encode//sum 12.65, render/graph/ms 12.01 |
| warm-up | 5.49 | 34.8 | pipelines/compile_ms 1431, pipelines/front_ms 1384, render/thread/ms 57.73, main/busy/ms 56.38, render/busy/ms 55.36, pipelines/driver_ms 47.04, stage/main/update 32.66, stage/render/prepare_assets 21.08, stage/main/post_update 17.35, encode//sum 16.71, stage/render/render 15.45, pipelines/wait_ms 13.99 |
| warm-up | 5.53 | 37.2 | pipelines/compile_ms 427, pipelines/front_ms 416, render/thread/ms 63.25, render/busy/ms 61.69, main/busy/ms 47.93, stage/render/prepare_assets 32.74, stage/main/update 23.10, stage/main/post_update 18.50, encode//sum 16.16, stage/render/render 15.05, render/graph/ms 12.76, stream/drive/ms 12.48 |
| warm-up | 5.57 | 39.9 | render/thread/ms 70.03, render/busy/ms 68.47, stage/render/prepare_assets 41.59, main/busy/ms 40.97, pipelines/compile_ms 27.10, pipelines/front_ms 25.90, stage/main/post_update 19.04, stage/main/update 16.21, stage/render/render 14.84, encode//sum 14.84, render/graph/ms 12.70, stage/render/graph_submit 10.10 |
| warm-up | 5.61 | 45.5 | pipelines/compile_ms 210, pipelines/front_ms 205, render/thread/ms 73.09, render/busy/ms 71.44, main/busy/ms 48.23, stage/render/prepare_assets 45.63, stage/main/post_update 23.39, stage/main/update 19.56, encode//sum 16.00, stage/render/render 15.69, render/graph/ms 13.36, stage/render/graph_submit 10.32 |
| warm-up | 6.13 | 31.6 | render/thread/ms 40.52, render/busy/ms 39.32, main/busy/ms 28.45, stage/render/prepare_assets 17.90, encode//sum 14.21, stage/main/update 12.57, stage/main/post_update 9.71, stage/render/render 9.30, render/graph/ms 7.66, stage/render/graph_submit 5.75, stream/drive/ms 4.27, encode/submit 2.94 |
| warm-up | 6.32 | 34.1 | render/thread/ms 44.25, render/busy/ms 42.55, main/busy/ms 39.18, encode//sum 24.57, stage/main/update 20.56, stage/render/render 16.07, render/graph/ms 13.67, stage/main/post_update 12.16, stage/render/graph_submit 9.96, stage/render/prepare_assets 8.07, stream/drive/ms 6.48, encode/submit 5.13 |
| warm-up | 6.38 | 30.7 | pipelines/compile_ms 75.51, pipelines/front_ms 71.83, render/thread/ms 37.66, render/busy/ms 36.57, main/busy/ms 29.36, stage/render/prepare_assets 19.35, encode//sum 15.19, stage/main/update 12.53, stage/main/post_update 11.38, stage/render/render 8.96, render/graph/ms 7.45, stage/render/graph_submit 5.40 |
| warm-up | 6.46 | 37.8 | main/busy/ms 44.02, render/thread/ms 34.59, render/busy/ms 33.31, stage/main/update 26.32, encode//sum 19.19, stream/drive/ms 13.40, stage/main/post_update 12.80, stage/render/render 12.52, stage/render/prepare_assets 12.37, render/graph/ms 10.73, stage/render/graph_submit 7.78, encode/finish_wall 4.55 |
| warm-up | 6.64 | 28.2 | render/thread/ms 44.77, render/busy/ms 43.64, main/busy/ms 26.75, stage/render/prepare_assets 26.12, encode//sum 18.35, stage/main/post_update 12.15, stage/main/update 11.54, stage/render/render 10.67, render/graph/ms 8.94, stage/render/graph_submit 6.79, stream/drive/ms 5.66, encode/finish_wall 3.47 |
| warm-up | 6.86 | 35.3 | main/busy/ms 44.43, render/thread/ms 32.55, render/busy/ms 31.09, stage/main/update 25.97, encode//sum 16.99, stream/drive/ms 15.41, stage/render/prepare_assets 13.42, stage/render/render 11.88, stage/main/post_update 11.03, render/graph/ms 9.84, stage/render/graph_submit 7.22, encode/finish_wall 3.94 |
| measure | 18.17 | 27.2 | main/busy/ms 34.94, encode//sum 23.34, stage/main/update 21.25, render/thread/ms 21.16, render/busy/ms 20.05, stream/drive/ms 17.36, stage/render/render 12.82, render/graph/ms 11.31, stage/main/post_update 11.03, stage/render/graph_submit 8.73, encode/submit 4.64, encode/finish_wall 4.17 |
| measure | 59.05 | 28.1 | render/thread/ms 38.00, render/busy/ms 36.80, main/busy/ms 26.82, encode//sum 20.02, stage/render/prepare_assets 14.66, stage/main/post_update 14.47, stage/render/render 12.38, render/graph/ms 10.77, stage/render/graph_submit 7.84, stage/main/update 7.38, encode/finish_wall 4.26, encode/submit 3.64 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_noslab\balmora_walk@2560x1440_off=meshslabs-2 --bench-baseline none --off meshslabs
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **81%** (min 52%) · 2580 MHz (min 2580) · memory 8001 MHz · 80.8 W · 74 °C (max 78) · VRAM 5302 MiB · throttled 100% of samples
