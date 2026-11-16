# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5640 frames over 65.6 s · **median 11.42 ms (88 fps)** · mean 11.64 · p95 14.38 · p99 16.66 (1% low 60 fps) · worst 33.96 ms · 42 spikes over 1.5× median · 23.8 stutters/min (work 25.6/min)
- warm-up: 325 frames over 9.5 s, worst 2665.0 ms
- hitches (> 28.5 ms): **4** measured, 44 in warm-up
- timed GPU passes: 7.23 ms of a 11.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.42 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.77/min, limit 1/min
- **FAIL** worst frame 33.96 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.005 | 2.086 | 2.309 | 0.96 | 0.176 |
| main_opaque_pass_3d | 1.579 | 1.643 | 2.044 | 0.96 | 0.067 |
| early prepass | 0.552 | 0.574 | 1.136 | 0.96 | 0.081 |
| atmosphere_luts | 0.310 | 0.322 | 0.431 | 0.96 | 0.003 |
| bloom | 0.309 | 0.322 | 0.326 | 0.96 | 0.064 |
| water_velocity | 0.274 | 0.285 | 0.354 | 0.96 | 0.002 |
| water_sspr | 0.199 | 0.207 | 0.206 | 0.96 | 0.002 |
| ssao | 0.187 | 0.194 | 0.196 | 0.96 | 0.007 |
| auxiliary_views | 0.166 | 0.172 | 0.639 | 0.96 | 0.158 |
| per_view_shadow_pass_cascade1 | 0.146 | 0.594 | 0.895 | 0.25 | 0.044 |
| uniform_allocation | 0.144 | 0.150 | 0.427 | 0.96 | 0.027 |
| atmos_froxel_inject | 0.143 | 0.148 | 0.148 | 0.96 | 0.002 |
| clouds_trace | 0.136 | 0.141 | 0.232 | 0.96 | 0.001 |
| water_fft | 0.130 | 0.135 | 0.141 | 0.96 | 0.003 |
| upscaling | 0.114 | 0.118 | 0.128 | 0.96 | 0.004 |
| clouds_composite | 0.104 | 0.108 | 0.110 | 0.96 | 0.002 |
| bin_unpacking | 0.099 | 0.103 | 0.124 | 0.96 | 0.019 |
| tonemapping | 0.083 | 0.086 | 0.090 | 0.96 | 0.003 |
| main_transmissive_pass_3d | 0.080 | 0.084 | 0.156 | 0.96 | 0.009 |
| clustering | 0.064 | 0.066 | 0.069 | 0.96 | 0.019 |
| render_sky | 0.050 | 0.052 | 0.053 | 0.96 | 0.002 |
| lightprobe_irradiance_map | 0.048 | 0.436 | 0.722 | 0.11 | 0.001 |
| clouds_resolve | 0.047 | 0.049 | 0.050 | 0.96 | 0.001 |
| surface_trail | 0.043 | 0.059 | 0.057 | 0.72 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.030 | 0.043 | 0.96 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.227 | 0.236 | 0.11 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.96 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.96 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.025 | 0.96 | 0.114 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.96 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.96 | 0.002 |
| lightprobe_downsampling_second_pass | 0.002 | 0.015 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_first_pass | 0.001 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.11 | 0.004 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.043 | 0.050 | 0.01 | 0.013 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.044 | 0.046 | 0.00 | 0.015 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.670 | 16.787 | 53.52 | 60176.2 |
| render/thread/ms | 10.304 | 12.932 | 26.17 | 58113.7 |
| render/busy/ms | 9.700 | 12.354 | 25.33 | 54707.0 |
| main/busy/ms | 9.396 | 11.759 | 29.44 | 52994.8 |
| stage/render/render | 6.495 | 8.346 | 16.81 | 36633.4 |
| stage/main/post_update | 5.905 | 7.843 | 11.66 | 33303.3 |
| render/graph/ms | 5.701 | 7.523 | 15.44 | 32155.8 |
| stage/render/graph_submit | 4.508 | 5.962 | 11.43 | 25425.8 |
| encode/submit | 2.403 | 3.305 | 5.86 | 13552.8 |
| stage/main/update | 2.225 | 2.714 | 22.63 | 12549.7 |
| encode/finish_wall | 2.190 | 2.958 | 8.27 | 12351.9 |
| stage/render/camera_driver | 1.138 | 1.596 | 3.99 | 6418.5 |
| stage/render/prepare_write_phase_buffers | 0.783 | 1.088 | 2.28 | 4413.9 |
| bigspace/total/ms | 0.623 | 1.947 | 3.37 | 3512.1 |
| stage/render/prepare_assets | 0.582 | 0.762 | 12.23 | 3280.7 |
| render/present/ms | 0.512 | 0.725 | 2.27 | 2886.2 |
| stage/render/prepare_resources | 0.468 | 0.584 | 1.38 | 2640.5 |
| stage/main/first | 0.397 | 0.524 | 5.07 | 2237.1 |
| stage/render/prepare_meshes | 0.395 | 0.530 | 1.82 | 2228.8 |
| stage/main/pre_update | 0.335 | 0.499 | 1.10 | 1891.4 |
| terrain/update/ms | 0.306 | 0.404 | 1.74 | 1728.5 |
| stage/render/specialize | 0.283 | 0.429 | 2.81 | 1596.0 |
| controller/feed/ms | 0.275 | 0.358 | 0.72 | 1550.6 |
| terrain/select/ms | 0.273 | 0.360 | 0.74 | 1539.0 |
| stage/render/prepare_bind_groups | 0.267 | 0.460 | 2.65 | 1507.2 |
| bigspace/low_precision/ms | 0.259 | 0.948 | 3.09 | 1459.8 |
| stream/drive/ms | 0.259 | 0.354 | 20.54 | 1458.5 |
| stage/main/fixed_main_loop | 0.233 | 0.387 | 1.46 | 1313.9 |
| stage/render/queue | 0.215 | 0.368 | 2.44 | 1210.1 |
| bigspace/high_precision/ms | 0.205 | 0.840 | 2.89 | 1154.8 |
| anim/evaluate/ms | 0.205 | 0.340 | 2.46 | 1154.6 |
| stream/tick/ms | 0.200 | 0.273 | 0.67 | 1127.2 |
| far/select/ms | 0.196 | 0.278 | 0.57 | 1106.5 |
| stage/render/create_views | 0.189 | 0.248 | 1.41 | 1063.4 |
| stage/render/extract_commands | 0.180 | 0.216 | 0.49 | 1014.7 |
| stage/render/prepare_flush | 0.177 | 0.359 | 0.84 | 995.7 |
| stage/render/prepare_batch_phases | 0.166 | 0.233 | 0.46 | 935.0 |
| anim/evaluate_cpu/ms | 0.146 | 0.195 | 0.52 | 823.2 |
| controller/move/ms | 0.142 | 0.273 | 0.63 | 802.2 |
| stage/render/prepare_views | 0.135 | 0.173 | 0.76 | 759.1 |
| stage/main/last | 0.122 | 0.195 | 1.57 | 688.8 |
| render/acquire/ms | 0.092 | 0.120 | 0.39 | 520.6 |
| stage/render/phase_sort | 0.080 | 0.109 | 0.29 | 450.5 |
| pipelines/compile_ms | 0.052 | 0.000 | 121.29 | 295.1 |
| physics/step/ms | 0.051 | 0.089 | 0.58 | 287.8 |
| pipelines/front_ms | 0.050 | 0.000 | 117.17 | 282.7 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.076 | 0.50 | 269.7 |
| physics/insert/ms | 0.035 | 0.376 | 0.87 | 196.8 |
| stage/render/cleanup | 0.033 | 0.098 | 0.76 | 188.2 |
| encode/resolve_diagnostics/sum | 0.032 | 0.045 | 0.32 | 180.8 |
| weather/map/ms | 0.026 | 0.034 | 0.23 | 148.4 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.023 | 0.35 | 87.6 |
| anim/apply/ms | 0.012 | 0.022 | 0.17 | 66.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.40 | 64.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.05 | 60.5 |
| controller/avoid/ms | 0.010 | 0.014 | 0.18 | 58.7 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 40.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.40 | 29.8 |
| script/input/ms | 0.005 | 0.006 | 0.08 | 28.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 26.8 |
| script/ui/ms | 0.005 | 0.006 | 0.09 | 25.7 |
| controller/write/ms | 0.003 | 0.004 | 0.06 | 19.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 14.5 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.54 | 12.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.1 |
| stage/render/prepare | 0.001 | 0.002 | 0.08 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 27 | 16 |
| controller/movers/count | count | 19 | 22 | 16 |
| crowd/c0/count | count | 26 | 42 | 15 |
| crowd/c0/joints/count | count | 1257 | 2204 | 707 |
| crowd/c0/meshes/count | count | 349 | 634 | 189 |
| crowd/c1/count | count | 146 | 156 | 146 |
| crowd/c1/joints/count | count | 5531 | 6011 | 5489 |
| crowd/c1/meshes/count | count | 1166 | 1297 | 1152 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 202 | 250 | 238 |
| crowd/joints_extracted/count | count | 4633 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4620 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.75 | 8.16 | 1.73 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.35 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.40 | 0.01 |
| encode/finish_critical | ms | 1.75 | 8.16 | 1.73 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.32 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.92 | 29.44 | 9.61 |
| gpu/timed/ms | ms | 8.69 | 12.05 | 9.61 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3392 | 5113 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3392 | 5113 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 208 | 19 |
| phases/shadow/bins | count | 4469 | 5382 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.2 k | 12.6 k | 5438 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 231 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2345 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 237 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 304 | 305 | 305 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 573 | 37 |
| preprocess/bind_groups_created | count | 13 | 573 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 132 | 230 | 123 |
| process/cpu_usage | % | 26.92 | 39.09 | 30.77 |
| process/mem_usage | GiB | 5.37 | 5.53 | 5.53 |
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
| started_load_count |  | 4917 | 5011 | 5011 |
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
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.51 | 56.60 | 33.54 |
| system/mem_usage | % | 67.38 | 68.19 | 66.84 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 68 | 255 | 69 |
| views/all/bins | count | 6980 | 8260 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.1 k | 25.0 k | 13.1 k |
| views/all/pipelines | count | 244 | 276 | 65 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 22.2 k | 13.4 k |
| views/cascade/batch_sets | count | 10 | 33 | 19 |
| views/cascade/bins | count | 2227 | 2861 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7660 | 9652 | 5438 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 538 | 9053 | 5438 |
| views/lamp_face/batch_sets | count | 0 | 198 | 0 |
| views/lamp_face/bins | count | 2232 | 2779 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2458 | 3075 | 0 |
| views/lamp_face/pipelines | count | 176 | 203 | 0 |
| views/lamp_face/work_items | count | 0 | 2930 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2882 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8389 | 13.0 k | 7612 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8554 | 13.2 k | 7920 |
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
| anim/bones_written/count | count | 59.27 | 294 | 334.3 k |
| anim/events/count | count | 0.10 | 4 | 578 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 3 | 19 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.84 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1485 | 4676 |
| encode//count | count | 39.81 | 271 | 224.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5640 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5640 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5640 |
| physics/inserts | count | 0.40 | 109 | 2263 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 161 | 2335 |
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
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 21 | 378 |
| stream/upload/bytes | bytes | 6002 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1316 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.5 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2665.0 | main/busy/ms 2634, stage/main/post_update 2528, stage/main/update 97.94, surface/advance_cpu_ms 68.65, stage/main/pre_update 2.97, stage/main/last 1.07, surface/cell_upload_ms 0.78, stage/main/first 0.72, stage/main/fixed_main_loop 0.46, stream/drive/ms 0.40, weather/map/ms 0.14, bigspace/total/ms 0.09 |
| warm-up | 3.42 | 751.9 | main/busy/ms 2768, stage/main/post_update 2551, render/thread/ms 721, render/busy/ms 547, stage/render/prepare_assets 510, stage/main/update 203, stage/render/prepare_views 172, render/acquire/ms 171, surface/advance_cpu_ms 68.65, stage/render/prepare_resources 22.46, stage/render/render 11.07, encode//sum 11.06 |
| warm-up | 3.50 | 87.4 | render/thread/ms 792, render/busy/ms 616, stage/render/prepare_assets 558, stage/render/prepare_views 173, render/acquire/ms 172, main/busy/ms 159, stage/main/update 112, stage/main/post_update 36.50, stage/render/prepare_resources 27.72, encode//sum 24.75, stage/render/render 23.86, pipelines/compile_ms 20.45 |
| warm-up | 3.65 | 125.5 | pipelines/wait_ms 3111, pipelines/compile_ms 937, pipelines/front_ms 879, render/thread/ms 143, render/busy/ms 138, main/busy/ms 78.93, pipelines/driver_ms 58.10, stage/render/prepare_assets 42.72, stage/render/prepare_resources 42.53, stage/main/post_update 41.85, pipelines/lock_ms 35.66, stage/render/prepare_flush 19.20 |
| warm-up | 3.74 | 85.1 | pipelines/wait_ms 6728, pipelines/compile_ms 1954, pipelines/front_ms 1863, render/thread/ms 208, render/busy/ms 202, main/busy/ms 150, stage/render/prepare_resources 94.64, pipelines/driver_ms 91.29, stage/main/update 79.47, stage/main/post_update 45.72, stage/render/prepare_assets 42.91, pipelines/lock_ms 39.71 |
| warm-up | 3.87 | 132.9 | pipelines/wait_ms 4096, pipelines/compile_ms 2780, pipelines/front_ms 2624, render/thread/ms 207, render/busy/ms 204, pipelines/driver_ms 156, main/busy/ms 129, stage/render/prepare_assets 113, stage/main/update 78.43, stage/render/prepare_resources 55.72, stage/main/post_update 39.98, encode//sum 17.62 |
| warm-up | 3.93 | 38.5 | pipelines/compile_ms 322, pipelines/front_ms 308, pipelines/wait_ms 194, render/thread/ms 58.74, render/busy/ms 57.10, stage/render/prepare_assets 40.30, main/busy/ms 33.51, stage/main/post_update 14.09, pipelines/driver_ms 14.02, terrain/update/ms 11.95, stage/main/update 10.15, stage/render/render 9.18 |
| warm-up | 3.96 | 28.7 | pipelines/compile_ms 109, pipelines/front_ms 102, render/thread/ms 63.31, render/busy/ms 61.45, stage/render/prepare_assets 30.40, main/busy/ms 28.20, encode//sum 16.30, stage/main/post_update 13.72, stage/render/render 13.53, stage/render/prepare_resources 12.47, render/graph/ms 11.18, pipelines/wait_ms 10.97 |
| warm-up | 4.08 | 74.6 | pipelines/compile_ms 863, pipelines/front_ms 826, main/busy/ms 65.46, render/thread/ms 54.30, stage/main/update 52.99, render/busy/ms 52.03, stream/drive/ms 46.63, pipelines/driver_ms 37.09, stage/render/prepare_assets 35.78, stage/render/render 11.54, render/graph/ms 8.77, encode//sum 8.15 |
| warm-up | 4.13 | 30.7 | pipelines/compile_ms 633, pipelines/front_ms 602, render/thread/ms 41.43, render/busy/ms 39.31, pipelines/driver_ms 30.86, main/busy/ms 27.53, stage/render/prepare_assets 20.39, stage/render/render 11.88, stage/main/post_update 10.24, stage/main/update 10.16, render/graph/ms 9.32, encode//sum 8.77 |
| warm-up | 4.17 | 41.8 | pipelines/compile_ms 726, pipelines/front_ms 690, render/thread/ms 60.09, render/busy/ms 58.31, stage/render/prepare_assets 44.34, pipelines/driver_ms 35.92, main/busy/ms 31.76, pipelines/wait_ms 14.72, stage/main/update 12.67, stage/main/post_update 12.02, stage/render/render 9.95, render/graph/ms 7.75 |
| warm-up | 4.21 | 42.5 | pipelines/compile_ms 596, pipelines/front_ms 575, render/thread/ms 71.09, render/busy/ms 69.46, stage/render/prepare_assets 55.62, main/busy/ms 37.69, pipelines/driver_ms 20.74, stage/main/update 16.53, pipelines/wait_ms 14.66, stage/main/post_update 14.07, stage/render/render 10.20, render/graph/ms 8.28 |
| warm-up | 4.25 | 35.1 | pipelines/compile_ms 575, pipelines/front_ms 555, render/thread/ms 65.99, render/busy/ms 63.55, stage/render/prepare_assets 44.48, main/busy/ms 32.72, pipelines/driver_ms 19.24, stage/main/update 13.78, stage/render/render 11.18, stage/main/post_update 10.83, render/graph/ms 8.43, encode//sum 7.04 |
| warm-up | 4.29 | 44.2 | pipelines/compile_ms 350, pipelines/front_ms 338, render/thread/ms 64.38, render/busy/ms 61.60, main/busy/ms 39.33, stage/render/prepare_assets 28.09, stage/main/update 22.34, stage/render/render 16.55, render/graph/ms 13.62, pipelines/driver_ms 12.87, stage/main/post_update 9.28, stage/render/graph_submit 8.59 |
| warm-up | 4.35 | 55.3 | pipelines/compile_ms 1230, pipelines/front_ms 1192, main/busy/ms 91.62, pipelines/wait_ms 77.58, render/thread/ms 69.77, render/busy/ms 66.48, stage/main/update 58.60, pipelines/driver_ms 37.87, stage/render/prepare_assets 28.94, stage/render/render 19.93, render/graph/ms 16.40, stream/drive/ms 11.81 |
| warm-up | 4.45 | 104.7 | pipelines/compile_ms 1900, pipelines/front_ms 1845, render/thread/ms 103, render/busy/ms 99.67, main/busy/ms 78.63, pipelines/wait_ms 77.87, pipelines/driver_ms 55.43, stage/render/render 52.48, render/graph/ms 48.93, stage/main/update 46.20, stage/render/camera_driver 36.31, stage/render/prepare_assets 31.31 |
| warm-up | 4.53 | 74.4 | pipelines/compile_ms 1657, pipelines/front_ms 1611, pipelines/wait_ms 219, render/thread/ms 124, render/busy/ms 122, encode//sum 97.30, stage/render/render 63.17, render/graph/ms 60.33, pipelines/driver_ms 46.45, main/busy/ms 42.35, stage/render/camera_driver 39.97, stage/main/update 26.50 |
| warm-up | 4.60 | 72.6 | pipelines/compile_ms 1769, pipelines/front_ms 1693, pipelines/wait_ms 292, render/thread/ms 120, render/busy/ms 118, encode//sum 111, pipelines/driver_ms 75.30, main/busy/ms 51.99, stage/main/update 34.11, stage/render/render 33.30, render/graph/ms 30.28, stage/render/prepare_assets 24.57 |
| warm-up | 4.67 | 63.9 | pipelines/compile_ms 2016, pipelines/front_ms 1934, pipelines/wait_ms 322, render/thread/ms 120, render/busy/ms 118, pipelines/driver_ms 81.86, main/busy/ms 49.82, stage/render/render 40.38, encode//sum 39.08, render/graph/ms 37.63, stage/render/prepare_assets 23.91, stage/render/graph_submit 20.67 |
| warm-up | 4.76 | 92.9 | pipelines/compile_ms 1809, pipelines/front_ms 1755, pipelines/wait_ms 280, render/thread/ms 148, render/busy/ms 146, main/busy/ms 108, pipelines/driver_ms 53.79, stage/main/update 49.41, stage/render/prepare_resources 45.75, stage/render/render 39.71, stage/main/post_update 39.59, render/graph/ms 36.73 |
| warm-up | 4.83 | 74.8 | pipelines/compile_ms 2135, pipelines/front_ms 2080, pipelines/wait_ms 435, render/thread/ms 141, render/busy/ms 139, main/busy/ms 112, stage/main/update 65.88, pipelines/driver_ms 55.30, stage/render/prepare_resources 38.44, stage/render/prepare_assets 36.68, stage/main/post_update 34.94, stream/drive/ms 24.62 |
| warm-up | 4.94 | 110.8 | pipelines/compile_ms 2300, pipelines/front_ms 2189, pipelines/wait_ms 432, encode//sum 248, render/thread/ms 132, render/busy/ms 130, main/busy/ms 114, pipelines/driver_ms 110, stage/main/update 62.34, stage/render/prepare_assets 57.10, stream/drive/ms 47.12, stage/render/render 44.45 |
| warm-up | 4.99 | 42.2 | pipelines/compile_ms 1088, pipelines/front_ms 1003, encode//sum 250, main/busy/ms 143, render/thread/ms 115, render/busy/ms 113, stage/main/update 92.96, pipelines/driver_ms 85.12, stream/drive/ms 74.85, stage/render/render 50.78, render/graph/ms 47.88, stage/render/graph_submit 42.65 |
| warm-up | 5.08 | 89.5 | pipelines/compile_ms 878, pipelines/front_ms 849, main/busy/ms 126, stage/main/update 104, render/thread/ms 80.72, stream/drive/ms 79.85, render/busy/ms 78.40, stage/render/prepare_assets 35.26, pipelines/driver_ms 28.94, stage/render/render 26.61, render/graph/ms 23.68, stage/render/graph_submit 19.17 |
| warm-up | 5.21 | 136.1 | pipelines/compile_ms 1476, pipelines/front_ms 1436, render/thread/ms 135, render/busy/ms 133, main/busy/ms 126, stage/main/update 101, stage/render/prepare_assets 70.91, stream/drive/ms 55.52, pipelines/driver_ms 40.42, stage/render/prepare_write_phase_buffers 30.25, stage/render/render 17.31, encode//sum 16.64 |
| warm-up | 5.27 | 56.7 | pipelines/compile_ms 1888, pipelines/front_ms 1837, render/thread/ms 135, render/busy/ms 133, main/busy/ms 109, stage/main/update 86.96, stage/render/prepare_assets 60.76, pipelines/driver_ms 51.15, stream/drive/ms 45.48, stage/render/render 32.02, render/graph/ms 29.51, stage/render/prepare_write_phase_buffers 29.40 |
| warm-up | 5.30 | 35.3 | pipelines/compile_ms 1028, pipelines/front_ms 996, main/busy/ms 76.98, render/thread/ms 74.78, render/busy/ms 72.86, stage/main/update 59.91, stream/drive/ms 37.99, stage/render/render 34.97, render/graph/ms 32.14, pipelines/driver_ms 31.85, stage/render/graph_submit 25.94, stage/render/prepare_assets 23.67 |
| warm-up | 5.34 | 36.7 | pipelines/compile_ms 83.98, pipelines/front_ms 79.34, render/thread/ms 59.37, render/busy/ms 57.21, main/busy/ms 53.47, stage/main/update 34.74, stage/render/render 17.60, encode//sum 16.93, render/graph/ms 14.85, stage/render/prepare_assets 14.14, stream/drive/ms 13.41, stage/main/post_update 11.81 |
| warm-up | 5.37 | 34.0 | render/thread/ms 58.28, pipelines/compile_ms 56.63, render/busy/ms 56.07, pipelines/front_ms 53.18, main/busy/ms 41.85, stage/main/update 20.68, stage/render/render 17.20, encode//sum 16.57, stage/main/post_update 14.84, render/graph/ms 14.65, stage/render/prepare_assets 12.23, stage/render/graph_submit 11.62 |
| warm-up | 5.50 | 34.4 | pipelines/compile_ms 582, pipelines/front_ms 570, render/thread/ms 52.16, render/busy/ms 50.48, stage/render/prepare_assets 25.76, main/busy/ms 21.68, encode//sum 16.31, stage/render/render 14.39, pipelines/driver_ms 12.84, render/graph/ms 12.14, stage/main/post_update 9.28, stage/render/graph_submit 8.90 |
| warm-up | 5.58 | 33.7 | pipelines/compile_ms 252, pipelines/front_ms 244, render/thread/ms 52.84, render/busy/ms 49.97, main/busy/ms 48.72, stage/main/post_update 27.25, stage/render/render 22.25, render/graph/ms 19.47, stage/render/graph_submit 14.27, encode//sum 14.03, stage/main/update 11.56, stage/render/prepare_assets 10.24 |
| warm-up | 5.63 | 42.1 | pipelines/compile_ms 513, pipelines/front_ms 497, render/thread/ms 71.18, render/busy/ms 64.70, main/busy/ms 61.58, stage/render/render 34.82, stage/main/post_update 33.54, render/graph/ms 28.19, encode//sum 16.99, pipelines/driver_ms 16.69, stage/main/update 14.65, stage/render/graph_submit 12.73 |
| warm-up | 5.68 | 59.0 | pipelines/compile_ms 1064, pipelines/front_ms 1030, render/thread/ms 94.12, render/busy/ms 88.33, main/busy/ms 84.66, stage/main/post_update 53.30, pipelines/driver_ms 33.44, stage/render/render 30.37, render/graph/ms 24.00, stage/render/prepare_assets 23.95, stage/main/update 19.00, bigspace/total/ms 18.02 |
| warm-up | 5.73 | 46.9 | pipelines/compile_ms 1031, pipelines/front_ms 1001, render/thread/ms 88.93, render/busy/ms 87.08, main/busy/ms 71.06, stage/render/prepare_assets 44.21, stage/main/post_update 40.75, pipelines/driver_ms 30.13, stage/main/update 20.42, encode//sum 16.58, bigspace/total/ms 16.13, stage/render/render 13.58 |
| warm-up | 5.77 | 42.5 | pipelines/compile_ms 825, pipelines/front_ms 804, render/thread/ms 70.51, render/busy/ms 69.01, stage/render/prepare_assets 48.11, main/busy/ms 42.39, pipelines/driver_ms 20.98, stage/main/update 20.91, encode//sum 17.70, stage/main/post_update 14.35, stage/render/render 12.01, render/graph/ms 9.87 |
| warm-up | 5.88 | 38.7 | pipelines/compile_ms 221, pipelines/front_ms 216, render/thread/ms 55.74, render/busy/ms 54.16, stage/render/prepare_assets 33.89, main/busy/ms 27.92, encode//sum 16.86, stage/main/post_update 11.91, stage/render/render 11.22, stage/main/update 11.16, render/graph/ms 8.95, stage/render/graph_submit 6.27 |
| warm-up | 5.93 | 30.0 | render/thread/ms 42.16, render/busy/ms 40.70, main/busy/ms 23.09, stage/render/prepare_assets 19.01, encode//sum 15.83, stage/render/render 12.80, render/graph/ms 10.79, stage/main/post_update 9.72, stage/main/update 8.69, stage/render/graph_submit 7.38, encode/submit 3.74, encode/finish_wall 3.70 |
| warm-up | 6.12 | 35.9 | encode//sum 52.19, render/thread/ms 42.20, render/busy/ms 40.84, stage/render/render 29.62, render/graph/ms 27.77, main/busy/ms 27.70, stage/render/graph_submit 25.54, encode/finish_wall 20.67, stage/main/update 17.15, stream/drive/ms 12.98, stage/main/post_update 6.96, encode/submit 4.98 |
| warm-up | 6.46 | 30.6 | render/thread/ms 38.93, render/busy/ms 37.80, main/busy/ms 27.69, stage/render/prepare_assets 17.50, encode//sum 15.99, stage/main/update 12.18, stage/main/post_update 10.17, stage/render/render 9.54, render/graph/ms 8.08, stage/render/graph_submit 6.10, stream/drive/ms 4.33, encode/finish_wall 3.28 |
| warm-up | 6.67 | 33.5 | main/busy/ms 44.58, render/thread/ms 42.36, render/busy/ms 40.72, stage/main/update 23.75, encode//sum 20.17, stage/main/post_update 15.38, stage/render/render 13.32, render/graph/ms 11.29, stage/render/graph_submit 7.45, stage/render/prepare_assets 6.57, stream/drive/ms 5.94, stage/render/prepare_write_phase_buffers 5.04 |
| warm-up | 6.70 | 32.8 | render/thread/ms 53.31, render/busy/ms 51.97, main/busy/ms 45.56, encode//sum 21.02, stage/main/update 19.43, stage/main/post_update 18.52, stage/render/prepare_assets 15.21, stage/render/render 12.65, render/graph/ms 10.58, stage/render/graph_submit 6.87, stream/drive/ms 6.63, stage/render/prepare_write_phase_buffers 4.96 |
| warm-up | 6.75 | 29.1 | pipelines/compile_ms 91.29, pipelines/front_ms 87.30, render/thread/ms 41.34, render/busy/ms 40.00, main/busy/ms 34.71, encode//sum 18.37, stage/main/update 15.52, stage/render/render 14.05, stage/render/prepare_assets 13.76, stage/main/post_update 12.66, render/graph/ms 12.40, stage/render/graph_submit 9.38 |
| warm-up | 6.84 | 28.9 | render/thread/ms 43.36, render/busy/ms 42.16, main/busy/ms 41.46, stage/main/update 20.34, stage/render/prepare_assets 19.22, encode//sum 17.90, stage/main/post_update 15.72, stage/render/render 11.28, render/graph/ms 9.45, stage/render/graph_submit 6.16, stream/drive/ms 4.71, encode/finish_wall 3.82 |
| warm-up | 7.24 | 38.5 | main/busy/ms 44.08, render/thread/ms 28.22, render/busy/ms 26.74, stage/main/update 23.34, encode//sum 21.29, stream/drive/ms 15.04, stage/render/render 14.59, stream/spawn/ms 13.71, stage/main/post_update 13.56, render/graph/ms 12.68, stage/render/graph_submit 9.02, stage/render/prepare_assets 5.51 |
| measure | 0.76 | 28.7 | main/busy/ms 33.74, render/thread/ms 27.35, render/busy/ms 26.10, encode//sum 25.00, stage/main/update 17.89, stage/render/render 14.06, stream/drive/ms 12.66, stage/main/post_update 12.60, render/graph/ms 12.31, stage/render/graph_submit 8.86, stage/render/prepare_assets 5.88, encode/submit 4.53 |
| measure | 18.17 | 34.0 | main/busy/ms 40.31, encode//sum 28.01, stage/main/update 24.74, render/thread/ms 23.95, render/busy/ms 22.57, stream/drive/ms 20.79, stage/render/render 14.58, stage/main/post_update 12.80, render/graph/ms 12.64, stage/render/graph_submit 9.81, encode/submit 5.00, encode/finish_wall 4.90 |
| measure | 59.04 | 32.2 | render/thread/ms 34.58, render/busy/ms 33.17, main/busy/ms 23.00, encode//sum 22.48, stage/render/render 14.69, stage/main/post_update 12.97, render/graph/ms 12.94, stage/render/prepare_assets 10.06, stage/render/graph_submit 9.81, stage/main/update 5.78, encode/submit 5.54, encode/finish_wall 4.38 |
| measure | 59.07 | 33.7 | render/thread/ms 37.50, render/busy/ms 36.40, stage/render/prepare_assets 20.27, main/busy/ms 18.93, encode//sum 14.51, stage/render/render 10.23, stage/main/post_update 10.13, render/graph/ms 8.87, stage/render/graph_submit 6.95, stage/main/update 4.89, encode/submit 3.85, encode/finish_wall 3.18 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_on\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **78%** (min 62%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.4 W · 73 °C (max 78) · VRAM 5953 MiB · throttled 100% of samples
