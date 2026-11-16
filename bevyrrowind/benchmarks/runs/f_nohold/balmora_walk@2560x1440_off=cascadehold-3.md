# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5586 frames over 65.6 s · **median 11.54 ms (87 fps)** · mean 11.75 · p95 14.16 · p99 16.35 (1% low 61 fps) · worst 31.01 ms · 40 spikes over 1.5× median · 34.7 stutters/min (work 33.8/min)
- warm-up: 324 frames over 9.2 s, worst 2695.4 ms
- hitches (> 28.8 ms): **3** measured, 42 in warm-up
- timed GPU passes: 7.77 ms of a 11.75 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.54 ms, limit 11 ms
- **FAIL** frame_ms: 38 stutters (> 1.5x moving median of 60) in 1.09 min = 34.73/min, limit 1/min
- pass worst frame 31.01 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.034 | 2.109 | 2.466 | 0.96 | 0.179 |
| main_opaque_pass_3d | 1.621 | 1.681 | 2.337 | 0.96 | 0.056 |
| per_view_shadow_pass_cascade1 | 0.584 | 0.606 | 0.946 | 0.96 | 0.043 |
| early prepass | 0.520 | 0.539 | 0.895 | 0.96 | 0.082 |
| bloom | 0.313 | 0.324 | 0.328 | 0.96 | 0.066 |
| atmosphere_luts | 0.311 | 0.322 | 0.430 | 0.96 | 0.003 |
| water_velocity | 0.282 | 0.292 | 0.354 | 0.96 | 0.002 |
| water_sspr | 0.202 | 0.209 | 0.206 | 0.96 | 0.002 |
| ssao | 0.192 | 0.200 | 0.203 | 0.96 | 0.007 |
| uniform_allocation | 0.174 | 0.180 | 0.473 | 0.96 | 0.030 |
| atmos_froxel_inject | 0.144 | 0.150 | 0.150 | 0.96 | 0.001 |
| clouds_trace | 0.137 | 0.142 | 0.234 | 0.96 | 0.001 |
| auxiliary_views | 0.132 | 0.137 | 0.575 | 0.96 | 0.122 |
| water_fft | 0.131 | 0.136 | 0.139 | 0.96 | 0.003 |
| bin_unpacking | 0.119 | 0.123 | 0.129 | 0.96 | 0.021 |
| upscaling | 0.118 | 0.122 | 0.132 | 0.96 | 0.004 |
| clouds_composite | 0.105 | 0.109 | 0.110 | 0.96 | 0.002 |
| tonemapping | 0.085 | 0.088 | 0.090 | 0.96 | 0.003 |
| main_transmissive_pass_3d | 0.082 | 0.085 | 0.162 | 0.96 | 0.008 |
| clustering | 0.063 | 0.066 | 0.068 | 0.96 | 0.019 |
| render_sky | 0.051 | 0.053 | 0.053 | 0.96 | 0.002 |
| clouds_resolve | 0.047 | 0.049 | 0.050 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.439 | 0.723 | 0.11 | 0.001 |
| early_mesh_preprocessing | 0.044 | 0.046 | 0.045 | 0.96 | 0.005 |
| surface_trail | 0.043 | 0.058 | 0.057 | 0.73 | 0.002 |
| clouds_shadow | 0.029 | 0.030 | 0.034 | 0.96 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.96 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.224 | 0.234 | 0.11 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.015 | 0.016 | 0.96 | 0.005 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.025 | 0.96 | 0.116 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.96 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.96 | 0.002 |
| lightprobe_copy | 0.002 | 0.017 | 0.017 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.001 | 0.053 | 0.049 | 0.01 | 0.014 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.018 | 0.021 | 0.01 | 0.010 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 11.220 | 16.000 | 49.06 | 62676.0 |
| render/thread/ms | 10.408 | 12.670 | 27.56 | 58141.3 |
| render/busy/ms | 9.791 | 12.069 | 26.69 | 54694.0 |
| main/busy/ms | 9.559 | 11.704 | 30.06 | 53397.4 |
| stage/render/render | 6.562 | 8.127 | 17.94 | 36654.7 |
| stage/main/post_update | 6.027 | 7.791 | 11.80 | 33667.0 |
| render/graph/ms | 5.751 | 7.286 | 16.93 | 32124.1 |
| stage/render/graph_submit | 4.578 | 5.837 | 14.50 | 25574.8 |
| encode/submit | 2.446 | 3.298 | 6.94 | 13663.1 |
| stage/main/update | 2.238 | 2.720 | 23.53 | 12501.7 |
| encode/finish_wall | 2.220 | 2.912 | 10.70 | 12402.5 |
| stage/render/camera_driver | 1.116 | 1.483 | 4.57 | 6231.5 |
| stage/render/prepare_write_phase_buffers | 0.853 | 1.127 | 2.50 | 4765.0 |
| bigspace/total/ms | 0.659 | 2.098 | 3.26 | 3679.3 |
| stage/render/prepare_assets | 0.565 | 0.749 | 10.60 | 3154.1 |
| render/present/ms | 0.521 | 0.726 | 2.37 | 2909.1 |
| stage/render/prepare_resources | 0.463 | 0.577 | 2.26 | 2587.8 |
| stage/main/first | 0.408 | 0.540 | 5.14 | 2280.9 |
| stage/render/prepare_meshes | 0.391 | 0.522 | 2.03 | 2183.1 |
| stage/main/pre_update | 0.340 | 0.511 | 1.68 | 1898.2 |
| terrain/update/ms | 0.310 | 0.403 | 1.77 | 1729.2 |
| controller/feed/ms | 0.283 | 0.374 | 0.72 | 1578.3 |
| stage/render/specialize | 0.279 | 0.416 | 2.83 | 1558.1 |
| bigspace/low_precision/ms | 0.278 | 1.221 | 2.97 | 1552.1 |
| terrain/select/ms | 0.276 | 0.359 | 0.73 | 1539.2 |
| stream/drive/ms | 0.256 | 0.355 | 21.43 | 1432.4 |
| stage/render/prepare_bind_groups | 0.248 | 0.415 | 3.45 | 1387.1 |
| stage/main/fixed_main_loop | 0.237 | 0.406 | 1.31 | 1322.8 |
| bigspace/high_precision/ms | 0.224 | 1.218 | 2.68 | 1248.6 |
| stage/render/queue | 0.213 | 0.364 | 2.53 | 1191.2 |
| anim/evaluate/ms | 0.203 | 0.323 | 3.28 | 1133.1 |
| stream/tick/ms | 0.198 | 0.268 | 0.87 | 1104.2 |
| far/select/ms | 0.192 | 0.267 | 0.57 | 1073.6 |
| stage/render/prepare_flush | 0.186 | 0.372 | 0.95 | 1037.4 |
| stage/render/create_views | 0.185 | 0.248 | 1.76 | 1031.0 |
| stage/render/extract_commands | 0.179 | 0.223 | 0.46 | 1000.7 |
| stage/render/prepare_batch_phases | 0.172 | 0.244 | 0.75 | 958.2 |
| anim/evaluate_cpu/ms | 0.147 | 0.193 | 0.51 | 823.2 |
| stage/render/prepare_views | 0.140 | 0.184 | 1.45 | 779.5 |
| controller/move/ms | 0.129 | 0.307 | 0.62 | 720.8 |
| stage/main/last | 0.126 | 0.210 | 1.59 | 704.5 |
| render/acquire/ms | 0.096 | 0.129 | 1.42 | 538.2 |
| stage/render/phase_sort | 0.084 | 0.120 | 0.71 | 470.0 |
| pipelines/compile_ms | 0.052 | 0.000 | 118.16 | 289.1 |
| physics/step/ms | 0.051 | 0.088 | 0.48 | 282.8 |
| pipelines/front_ms | 0.050 | 0.000 | 114.25 | 277.3 |
| stage/render/prepare_collect_phase_buffers | 0.050 | 0.080 | 0.35 | 277.2 |
| stage/render/cleanup | 0.035 | 0.110 | 0.54 | 194.9 |
| physics/insert/ms | 0.034 | 0.376 | 0.93 | 190.4 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.64 | 177.8 |
| weather/map/ms | 0.026 | 0.034 | 0.28 | 146.3 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.60 | 85.3 |
| anim/apply/ms | 0.012 | 0.020 | 0.31 | 65.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.63 | 63.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.14 | 59.9 |
| controller/avoid/ms | 0.011 | 0.014 | 0.11 | 59.6 |
| script/update/ms | 0.007 | 0.010 | 0.31 | 41.7 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 30.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.13 | 29.0 |
| script/input/ms | 0.005 | 0.006 | 0.20 | 28.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 25.4 |
| controller/write/ms | 0.004 | 0.005 | 0.16 | 20.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 14.6 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.91 | 11.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.14 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.17 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.09 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1303 | 2262 | 641 |
| crowd/c0/meshes/count | count | 354 | 646 | 174 |
| crowd/c1/count | count | 145 | 154 | 147 |
| crowd/c1/joints/count | count | 5507 | 5895 | 5555 |
| crowd/c1/meshes/count | count | 1160 | 1267 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 188 | 236 | 223 |
| crowd/joints_extracted/count | count | 4286 | 9879 | 6179 |
| crowd/joints_uploaded/count | count | 4266 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.80 | 10.19 | 1.99 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.60 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.63 | 0.01 |
| encode/finish_critical | ms | 1.80 | 10.19 | 1.99 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.64 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.99 | 30.06 | 9.27 |
| gpu/timed/ms | ms | 8.78 | 12.08 | 9.27 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 192 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 836 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 192 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 836 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1289 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3478 | 5110 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1289 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3478 | 5110 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 25 | 218 | 19 |
| phases/shadow/bins | count | 4270 | 5348 | 1793 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9525 | 12.0 k | 4885 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 194 | 237 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2664 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 184 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 10 | 594 | 6 |
| preprocess/bind_groups_created | count | 10 | 594 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 160 | 260 | 154 |
| process/cpu_usage | % | 27.40 | 38.57 | 33.66 |
| process/mem_usage | GiB | 5.39 | 5.64 | 5.63 |
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
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.22 | 70.34 | 39.00 |
| system/mem_usage | % | 68.44 | 69.23 | 69.21 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 75 | 265 | 69 |
| views/all/bins | count | 6916 | 8272 | 5291 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.6 k | 24.5 k | 12.5 k |
| views/all/pipelines | count | 239 | 283 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 16.3 k | 21.9 k | 12.8 k |
| views/cascade/batch_sets | count | 24 | 33 | 19 |
| views/cascade/bins | count | 2081 | 2706 | 1793 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7015 | 9054 | 4885 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 7015 | 9054 | 4885 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2167 | 2775 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2422 | 3073 | 0 |
| views/lamp_face/pipelines | count | 170 | 205 | 0 |
| views/lamp_face/work_items | count | 0 | 2933 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2870 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8617 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8772 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 53.87 | 219 | 300.9 k |
| anim/events/count | count | 0.10 | 3 | 556 |
| assets/Image/added | count | 0.11 | 37 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.73 | 152 | 4061 |
| assets/Mesh/removed | count | 0.87 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.88 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.84 | 1386 | 4675 |
| encode//count | count | 39.10 | 273 | 218.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5586 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5586 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5586 |
| physics/inserts | count | 0.40 | 105 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.70 | 2 | 3938 |
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
| stream/spawn/items | count | 0.07 | 18 | 378 |
| stream/upload/bytes | bytes | 6060 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1329 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.8 ms; slowest 45, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.70 | 2695.4 | main/busy/ms 2665, stage/main/post_update 2562, stage/main/update 96.98, surface/advance_cpu_ms 69.21, stage/main/pre_update 2.51, stage/main/last 1.15, surface/cell_upload_ms 0.73, stage/main/fixed_main_loop 0.62, stage/main/first 0.51, stream/drive/ms 0.45, weather/map/ms 0.11, stream/tick/ms 0.09 |
| warm-up | 3.41 | 716.5 | main/busy/ms 2814, stage/main/post_update 2587, render/thread/ms 684, render/busy/ms 523, stage/render/prepare_assets 502, stage/main/update 214, stage/render/prepare_views 160, render/acquire/ms 159, surface/advance_cpu_ms 69.21, encode//sum 13.25, stage/render/render 9.70, stage/render/prepare_resources 7.88 |
| warm-up | 3.47 | 58.6 | render/thread/ms 726, render/busy/ms 564, stage/render/prepare_assets 523, main/busy/ms 168, stage/render/prepare_views 160, render/acquire/ms 159, stage/main/update 121, pipelines/compile_ms 76.51, pipelines/front_ms 72.16, stage/main/post_update 35.95, stage/render/render 22.18, terrain/update/ms 19.72 |
| warm-up | 3.51 | 34.9 | pipelines/compile_ms 217, pipelines/front_ms 201, pipelines/wait_ms 112, render/thread/ms 76.12, render/busy/ms 66.84, main/busy/ms 52.95, stage/render/render 38.00, stage/main/post_update 29.02, render/graph/ms 28.23, terrain/update/ms 24.31, stage/render/prepare_assets 22.17, pipelines/lock_ms 21.33 |
| warm-up | 3.61 | 106.3 | pipelines/wait_ms 2324, pipelines/compile_ms 913, pipelines/front_ms 863, render/thread/ms 139, render/busy/ms 130, main/busy/ms 63.80, stage/render/render 58.07, pipelines/driver_ms 49.98, render/graph/ms 49.07, stage/render/prepare_resources 44.36, stage/main/post_update 41.10, stage/render/graph_submit 32.85 |
| warm-up | 3.70 | 86.7 | pipelines/wait_ms 6017, pipelines/compile_ms 1916, pipelines/front_ms 1824, render/thread/ms 161, render/busy/ms 159, pipelines/driver_ms 91.77, stage/render/prepare_resources 61.90, main/busy/ms 61.82, stage/render/render 39.74, render/graph/ms 37.36, stage/main/post_update 36.18, stage/render/graph_submit 35.11 |
| warm-up | 3.74 | 38.5 | pipelines/wait_ms 3973, pipelines/compile_ms 1763, pipelines/front_ms 1678, main/busy/ms 110, render/thread/ms 91.35, render/busy/ms 88.75, pipelines/driver_ms 85.02, stage/main/update 82.51, stage/render/prepare_assets 41.53, stage/render/prepare_resources 21.10, stage/main/post_update 19.27, terrain/update/ms 17.12 |
| warm-up | 3.88 | 145.0 | pipelines/compile_ms 1832, pipelines/front_ms 1586, pipelines/wait_ms 417, encode//sum 416, pipelines/driver_ms 246, render/thread/ms 134, render/busy/ms 131, main/busy/ms 100, stage/render/prepare_assets 74.79, stage/main/update 74.36, stage/render/render 50.68, render/graph/ms 48.29 |
| warm-up | 3.94 | 36.6 | pipelines/compile_ms 1047, pipelines/front_ms 1012, pipelines/wait_ms 74.00, render/thread/ms 48.64, render/busy/ms 46.59, main/busy/ms 38.75, pipelines/driver_ms 34.46, encode//sum 22.81, stage/main/post_update 21.79, stage/render/prepare_assets 20.89, stage/render/render 18.15, render/graph/ms 15.57 |
| warm-up | 4.08 | 74.5 | pipelines/compile_ms 973, pipelines/front_ms 937, render/thread/ms 59.22, main/busy/ms 57.61, render/busy/ms 57.44, stage/main/update 46.05, stream/drive/ms 41.97, stage/render/prepare_assets 39.46, pipelines/driver_ms 36.04, stage/render/render 10.58, encode//sum 8.97, render/graph/ms 8.26 |
| warm-up | 4.13 | 36.2 | pipelines/compile_ms 465, pipelines/front_ms 448, render/thread/ms 41.08, render/busy/ms 39.22, main/busy/ms 38.75, stage/render/prepare_assets 22.08, pipelines/driver_ms 17.20, stage/main/update 14.28, stage/render/render 10.67, stream/drive/ms 10.09, render/graph/ms 8.47, stage/main/post_update 7.10 |
| warm-up | 4.18 | 47.7 | pipelines/compile_ms 676, pipelines/front_ms 651, render/thread/ms 68.01, render/busy/ms 65.62, main/busy/ms 56.28, stage/render/prepare_assets 42.94, stage/main/update 27.19, pipelines/driver_ms 25.12, stream/drive/ms 23.19, stage/render/render 17.79, render/graph/ms 14.97, stage/main/post_update 12.93 |
| warm-up | 4.22 | 39.4 | pipelines/compile_ms 485, pipelines/front_ms 466, render/thread/ms 70.32, render/busy/ms 67.27, main/busy/ms 49.43, stage/render/prepare_assets 38.27, stage/main/update 29.51, stream/drive/ms 24.42, stage/render/render 23.12, render/graph/ms 19.23, pipelines/driver_ms 18.50, stage/main/post_update 13.47 |
| warm-up | 4.28 | 65.0 | pipelines/compile_ms 305, pipelines/front_ms 294, render/thread/ms 80.28, render/busy/ms 77.53, main/busy/ms 46.33, stage/render/prepare_assets 29.97, stage/main/post_update 19.61, stage/main/update 18.03, stage/render/prepare_resources 16.53, stage/render/render 15.08, render/graph/ms 11.90, pipelines/driver_ms 10.81 |
| warm-up | 4.39 | 111.6 | pipelines/compile_ms 1241, pipelines/front_ms 1209, render/thread/ms 132, render/busy/ms 96.95, main/busy/ms 73.30, stage/render/prepare_assets 53.01, stage/main/update 45.51, stage/render/render 42.51, render/present/ms 34.74, pipelines/driver_ms 32.63, stage/main/post_update 19.87, stage/render/prepare_resources 15.97 |
| warm-up | 4.48 | 82.4 | pipelines/compile_ms 3041, pipelines/front_ms 2892, pipelines/driver_ms 149, render/thread/ms 139, render/busy/ms 104, pipelines/wait_ms 84.39, stage/render/prepare_assets 79.34, main/busy/ms 65.01, stage/render/render 44.95, stage/main/update 42.37, render/present/ms 34.71, stage/main/post_update 11.49 |
| warm-up | 4.57 | 92.1 | pipelines/compile_ms 2369, pipelines/front_ms 2235, pipelines/wait_ms 217, render/thread/ms 141, render/busy/ms 140, pipelines/driver_ms 134, main/busy/ms 87.88, stage/render/prepare_assets 57.39, stage/main/post_update 54.48, encode//sum 54.44, stage/render/prepare_resources 23.03, stage/render/prepare_write_phase_buffers 22.79 |
| warm-up | 4.63 | 63.0 | pipelines/compile_ms 1384, pipelines/front_ms 1337, pipelines/wait_ms 568, render/thread/ms 136, main/busy/ms 136, render/busy/ms 134, stage/main/post_update 65.71, encode//sum 62.29, stage/main/update 46.70, pipelines/driver_ms 46.68, stage/render/prepare_write_phase_buffers 29.14, stage/render/prepare_assets 27.06 |
| warm-up | 4.74 | 110.3 | pipelines/compile_ms 2428, pipelines/front_ms 2319, pipelines/wait_ms 1043, render/thread/ms 137, render/busy/ms 135, pipelines/driver_ms 108, main/busy/ms 105, stage/main/update 52.87, encode//sum 33.17, stage/render/prepare_resources 28.94, stage/main/post_update 25.12, stage/render/prepare_assets 25.04 |
| warm-up | 4.78 | 37.1 | pipelines/compile_ms 1765, pipelines/front_ms 1678, pipelines/wait_ms 678, render/thread/ms 110, render/busy/ms 108, pipelines/driver_ms 87.44, main/busy/ms 50.66, stage/render/prepare_resources 28.83, encode//sum 25.28, stage/main/update 24.64, stage/render/prepare_assets 23.42, stage/render/render 18.62 |
| warm-up | 4.84 | 61.4 | pipelines/compile_ms 1552, pipelines/front_ms 1509, pipelines/wait_ms 456, main/busy/ms 80.52, render/thread/ms 61.73, stage/main/update 60.37, render/busy/ms 59.67, stream/drive/ms 52.27, pipelines/driver_ms 43.22, stage/render/prepare_assets 35.47, encode//sum 13.36, stage/render/render 13.23 |
| warm-up | 4.91 | 65.1 | pipelines/compile_ms 2102, pipelines/front_ms 2044, pipelines/wait_ms 908, main/busy/ms 111, stage/main/update 91.52, render/thread/ms 71.77, render/busy/ms 69.04, pipelines/driver_ms 57.70, stream/drive/ms 52.93, stage/render/prepare_assets 29.46, stage/render/render 28.38, render/graph/ms 24.62 |
| warm-up | 4.98 | 79.4 | pipelines/compile_ms 1241, pipelines/front_ms 1202, pipelines/wait_ms 548, main/busy/ms 148, stage/main/update 125, render/thread/ms 83.13, render/busy/ms 80.46, stream/drive/ms 49.64, pipelines/driver_ms 39.02, stage/render/render 31.90, render/graph/ms 28.53, pipelines/lock_ms 26.80 |
| warm-up | 5.09 | 107.6 | pipelines/compile_ms 1878, pipelines/front_ms 1823, main/busy/ms 128, stage/main/update 102, render/thread/ms 91.36, render/busy/ms 89.32, pipelines/driver_ms 55.40, stream/drive/ms 49.98, pipelines/wait_ms 44.46, stage/render/prepare_assets 42.50, pipelines/lock_ms 35.66, encode//sum 20.73 |
| warm-up | 5.12 | 32.2 | pipelines/compile_ms 1936, pipelines/front_ms 1879, main/busy/ms 84.47, render/thread/ms 70.04, render/busy/ms 68.35, stage/main/update 63.33, pipelines/driver_ms 57.39, stream/drive/ms 43.46, stage/render/prepare_assets 35.99, pipelines/wait_ms 22.50, encode//sum 17.31, pipelines/lock_ms 14.35 |
| warm-up | 5.19 | 65.9 | pipelines/compile_ms 768, pipelines/front_ms 743, main/busy/ms 71.11, stage/main/update 58.42, render/thread/ms 52.40, render/busy/ms 50.91, stream/drive/ms 42.76, stage/render/prepare_assets 28.87, pipelines/driver_ms 24.92, encode//sum 14.26, stage/render/render 10.03, render/graph/ms 8.01 |
| warm-up | 5.22 | 32.4 | pipelines/compile_ms 291, pipelines/front_ms 280, main/busy/ms 58.14, render/thread/ms 53.96, render/busy/ms 51.74, stage/main/update 44.70, stream/drive/ms 33.78, stage/render/prepare_assets 18.47, encode//sum 16.34, stage/render/render 13.77, render/graph/ms 11.43, pipelines/driver_ms 10.86 |
| warm-up | 5.26 | 36.8 | pipelines/compile_ms 217, pipelines/front_ms 210, main/busy/ms 54.43, render/thread/ms 45.12, render/busy/ms 42.90, stage/main/update 39.34, stream/drive/ms 33.60, encode//sum 15.91, stage/render/render 14.61, render/graph/ms 12.39, stage/render/prepare_assets 10.03, stage/render/graph_submit 9.01 |
| warm-up | 5.38 | 29.4 | pipelines/compile_ms 228, pipelines/front_ms 220, render/thread/ms 48.17, render/busy/ms 46.08, main/busy/ms 35.44, encode//sum 16.80, stage/render/render 15.45, stage/main/update 14.01, render/graph/ms 12.72, stage/main/post_update 12.15, stage/render/prepare_assets 11.92, stage/render/graph_submit 9.01 |
| warm-up | 5.42 | 38.1 | pipelines/compile_ms 547, pipelines/front_ms 531, render/thread/ms 53.98, render/busy/ms 52.33, main/busy/ms 38.09, stage/render/prepare_assets 26.46, stage/main/update 19.64, pipelines/driver_ms 16.09, encode//sum 15.53, stage/render/render 14.05, render/graph/ms 11.68, stage/main/post_update 9.41 |
| warm-up | 5.52 | 48.1 | pipelines/compile_ms 646, pipelines/front_ms 623, render/thread/ms 62.01, render/busy/ms 60.43, main/busy/ms 49.27, stage/main/update 28.81, stage/render/prepare_assets 26.00, stage/render/render 23.48, pipelines/driver_ms 22.38, render/graph/ms 21.13, encode//sum 17.02, stage/main/post_update 15.26 |
| warm-up | 5.58 | 32.7 | render/thread/ms 50.40, render/busy/ms 48.83, pipelines/compile_ms 41.82, pipelines/front_ms 39.81, main/busy/ms 36.63, stage/render/prepare_assets 21.52, stage/main/post_update 18.37, encode//sum 17.63, stage/render/render 13.03, stage/main/update 12.34, render/graph/ms 11.05, stage/render/graph_submit 8.21 |
| warm-up | 5.63 | 30.1 | pipelines/compile_ms 570, pipelines/front_ms 557, render/thread/ms 43.31, render/busy/ms 41.64, main/busy/ms 35.30, encode//sum 17.54, stage/main/update 17.22, stage/render/render 14.81, stage/render/prepare_assets 14.51, pipelines/driver_ms 13.23, stage/main/post_update 13.06, render/graph/ms 12.59 |
| warm-up | 5.71 | 35.9 | pipelines/compile_ms 373, pipelines/front_ms 364, render/thread/ms 50.39, render/busy/ms 49.06, stage/render/prepare_assets 26.64, main/busy/ms 26.00, encode//sum 17.51, stage/main/post_update 11.58, stage/render/render 10.77, stage/main/update 9.39, pipelines/driver_ms 9.05, render/graph/ms 8.90 |
| warm-up | 5.77 | 30.5 | pipelines/compile_ms 169, pipelines/front_ms 161, render/thread/ms 46.38, render/busy/ms 44.90, stage/render/prepare_assets 24.93, main/busy/ms 24.54, encode//sum 16.63, stage/render/render 12.31, stage/main/post_update 12.09, render/graph/ms 10.12, pipelines/driver_ms 8.29, stage/render/graph_submit 7.68 |
| warm-up | 6.27 | 30.7 | render/thread/ms 38.77, render/busy/ms 37.66, main/busy/ms 27.11, encode//sum 16.64, stage/render/prepare_assets 16.45, stage/main/update 11.26, stage/main/post_update 9.96, stage/render/render 9.45, render/graph/ms 7.95, stage/render/graph_submit 6.10, stream/drive/ms 3.89, encode/finish_wall 3.10 |
| warm-up | 6.49 | 31.1 | pipelines/compile_ms 79.07, pipelines/front_ms 75.15, render/thread/ms 39.10, main/busy/ms 37.77, render/busy/ms 37.53, encode//sum 24.32, stage/main/update 18.28, stage/render/render 14.96, render/graph/ms 12.73, stage/main/post_update 12.19, stage/render/graph_submit 9.74, stage/render/prepare_assets 9.09 |
| warm-up | 6.52 | 29.7 | pipelines/compile_ms 79.07, pipelines/front_ms 75.15, render/thread/ms 40.51, render/busy/ms 39.35, main/busy/ms 30.11, stage/render/prepare_assets 20.34, encode//sum 16.61, stage/main/update 13.23, stage/main/post_update 10.33, stage/render/render 8.87, render/graph/ms 7.27, stage/render/graph_submit 5.20 |
| warm-up | 6.55 | 29.2 | render/thread/ms 48.42, render/busy/ms 46.75, main/busy/ms 38.13, encode//sum 24.71, stage/render/render 16.91, stage/main/update 16.47, stage/render/prepare_assets 15.63, stage/main/post_update 15.52, render/graph/ms 14.75, stage/render/graph_submit 11.06, encode/submit 6.82, encode/finish_wall 4.41 |
| warm-up | 6.61 | 41.7 | main/busy/ms 44.16, render/thread/ms 35.10, render/busy/ms 33.85, stage/main/update 26.47, encode//sum 18.35, stage/render/prepare_assets 15.78, stream/drive/ms 15.02, stage/main/post_update 12.57, stage/render/render 9.82, render/graph/ms 7.97, stage/render/graph_submit 5.79, encode/finish_wall 3.55 |
| warm-up | 6.79 | 30.5 | render/thread/ms 44.45, render/busy/ms 43.33, main/busy/ms 27.23, stage/render/prepare_assets 25.15, encode//sum 19.63, stage/main/post_update 12.40, stage/render/render 11.39, stage/main/update 11.05, render/graph/ms 9.82, stage/render/graph_submit 7.52, stream/drive/ms 6.11, encode/submit 3.95 |
| warm-up | 6.99 | 35.2 | main/busy/ms 40.90, render/thread/ms 25.60, render/busy/ms 24.39, encode//sum 22.15, stage/main/update 20.94, stage/main/post_update 14.14, stream/drive/ms 13.99, stream/spawn/ms 12.77, stage/render/render 12.66, render/graph/ms 11.07, stage/render/graph_submit 8.55, stage/render/prepare_assets 4.88 |
| measure | 18.18 | 31.0 | main/busy/ms 39.62, stage/main/update 25.35, encode//sum 25.28, render/thread/ms 24.20, render/busy/ms 23.07, stream/drive/ms 21.69, stage/render/render 14.11, render/graph/ms 12.25, stage/main/post_update 11.43, stage/render/graph_submit 9.57, encode/submit 5.45, encode/finish_wall 4.18 |
| measure | 51.69 | 29.1 | render/thread/ms 41.68, render/busy/ms 39.95, encode//sum 35.21, main/busy/ms 32.01, stage/render/render 26.62, render/graph/ms 24.32, stage/render/graph_submit 20.12, stage/main/post_update 18.86, encode/finish_wall 13.88, stage/main/update 9.66, encode/submit 6.41, stage/render/camera_driver 3.98 |
| measure | 59.04 | 30.9 | render/thread/ms 42.23, render/busy/ms 40.90, encode//sum 28.66, main/busy/ms 28.10, stage/main/post_update 15.63, stage/render/render 15.50, stage/render/prepare_assets 14.53, render/graph/ms 13.62, stage/render/graph_submit 10.31, stage/main/update 7.64, encode/finish_wall 5.96, encode/submit 4.42 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_nohold\balmora_walk@2560x1440_off=cascadehold-3 --bench-baseline none --off cascadehold
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **82%** (min 66%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.8 W · 73 °C (max 77) · VRAM 6018 MiB · throttled 100% of samples
