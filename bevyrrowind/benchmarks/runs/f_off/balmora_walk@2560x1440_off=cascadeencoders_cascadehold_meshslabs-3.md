# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs,cascadeencoders,cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5061 frames over 65.7 s · **median 12.47 ms (80 fps)** · mean 12.97 · p95 16.27 · p99 20.68 (1% low 48 fps) · worst 36.78 ms · 80 spikes over 1.5× median · 38.4 stutters/min (work 37.5/min)
- warm-up: 324 frames over 9.5 s, worst 2695.9 ms
- hitches (> 31.2 ms): **4** measured, 43 in warm-up
- timed GPU passes: 7.42 ms of a 12.97 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.47 ms, limit 11 ms
- **FAIL** frame_ms: 42 stutters (> 1.5x moving median of 60) in 1.09 min = 38.38/min, limit 1/min
- **FAIL** worst frame 36.78 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.069 | 2.075 | 2.115 | 1.00 | 0.178 |
| main_opaque_pass_3d | 1.660 | 1.664 | 2.061 | 1.00 | 0.054 |
| early prepass | 0.525 | 0.526 | 0.751 | 1.00 | 0.080 |
| bloom | 0.319 | 0.320 | 0.326 | 1.00 | 0.066 |
| atmosphere_luts | 0.313 | 0.314 | 0.373 | 1.00 | 0.003 |
| water_velocity | 0.280 | 0.280 | 0.353 | 1.00 | 0.002 |
| water_sspr | 0.205 | 0.206 | 0.205 | 1.00 | 0.002 |
| auxiliary_views | 0.195 | 0.195 | 0.727 | 1.00 | 0.181 |
| ssao | 0.192 | 0.193 | 0.195 | 1.00 | 0.007 |
| uniform_allocation | 0.188 | 0.188 | 0.477 | 1.00 | 0.031 |
| atmos_froxel_inject | 0.149 | 0.149 | 0.151 | 1.00 | 0.001 |
| clouds_trace | 0.142 | 0.142 | 0.236 | 1.00 | 0.001 |
| bin_unpacking | 0.135 | 0.135 | 0.144 | 1.00 | 0.024 |
| water_fft | 0.133 | 0.133 | 0.139 | 1.00 | 0.003 |
| upscaling | 0.120 | 0.120 | 0.128 | 1.00 | 0.004 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 1.00 | 0.002 |
| tonemapping | 0.087 | 0.087 | 0.090 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.083 | 0.084 | 0.163 | 1.00 | 0.008 |
| clustering | 0.065 | 0.065 | 0.069 | 1.00 | 0.019 |
| render_sky | 0.052 | 0.052 | 0.053 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.422 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.046 | 0.057 | 0.057 | 0.81 | 0.002 |
| early_mesh_preprocessing | 0.045 | 0.045 | 0.049 | 1.00 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.222 | 0.233 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.027 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.026 | 1.00 | 0.115 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 13.279 | 20.868 | 53.80 | 67205.0 |
| render/thread/ms | 11.703 | 14.812 | 32.99 | 59229.2 |
| render/busy/ms | 11.125 | 14.206 | 31.67 | 56302.6 |
| main/busy/ms | 9.795 | 12.734 | 29.51 | 49573.5 |
| stage/render/render | 7.718 | 10.085 | 21.02 | 39058.9 |
| render/graph/ms | 6.943 | 9.249 | 19.36 | 35140.1 |
| stage/main/post_update | 6.292 | 8.784 | 16.03 | 31842.8 |
| stage/render/graph_submit | 5.708 | 7.576 | 15.66 | 28889.5 |
| encode/finish_wall | 3.260 | 4.543 | 8.87 | 16500.5 |
| encode/submit | 2.499 | 3.483 | 7.05 | 12645.1 |
| stage/main/update | 2.207 | 2.747 | 23.13 | 11168.3 |
| stage/render/camera_driver | 1.175 | 1.674 | 4.74 | 5948.2 |
| stage/render/prepare_write_phase_buffers | 0.955 | 1.282 | 3.12 | 4832.8 |
| bigspace/total/ms | 0.603 | 2.082 | 5.43 | 3052.6 |
| stage/render/prepare_assets | 0.582 | 0.753 | 9.36 | 2944.1 |
| render/present/ms | 0.484 | 0.596 | 1.83 | 2451.8 |
| stage/main/first | 0.414 | 0.553 | 6.08 | 2097.1 |
| stage/render/prepare_meshes | 0.396 | 0.520 | 1.63 | 2004.5 |
| stage/render/prepare_resources | 0.395 | 0.505 | 0.92 | 1999.2 |
| stage/main/pre_update | 0.331 | 0.497 | 1.14 | 1673.0 |
| terrain/update/ms | 0.316 | 0.422 | 2.33 | 1600.0 |
| stage/render/specialize | 0.311 | 0.461 | 2.99 | 1573.7 |
| controller/feed/ms | 0.283 | 0.369 | 0.73 | 1433.5 |
| terrain/select/ms | 0.282 | 0.378 | 0.76 | 1427.1 |
| stage/render/queue | 0.272 | 0.445 | 3.18 | 1375.1 |
| stage/render/prepare_bind_groups | 0.261 | 0.468 | 2.82 | 1323.2 |
| bigspace/low_precision/ms | 0.261 | 0.733 | 4.27 | 1322.3 |
| stream/drive/ms | 0.256 | 0.347 | 21.28 | 1295.5 |
| stage/main/fixed_main_loop | 0.250 | 0.385 | 1.27 | 1266.7 |
| far/select/ms | 0.193 | 0.260 | 0.62 | 977.0 |
| stream/tick/ms | 0.192 | 0.263 | 0.63 | 972.5 |
| anim/evaluate/ms | 0.188 | 0.291 | 2.98 | 953.5 |
| bigspace/high_precision/ms | 0.188 | 0.697 | 4.97 | 950.8 |
| stage/render/prepare_flush | 0.178 | 0.361 | 0.82 | 899.9 |
| stage/render/prepare_batch_phases | 0.173 | 0.241 | 0.67 | 875.7 |
| stage/render/create_views | 0.172 | 0.230 | 0.90 | 868.3 |
| stage/render/extract_commands | 0.169 | 0.206 | 0.45 | 856.4 |
| anim/evaluate_cpu/ms | 0.148 | 0.193 | 0.50 | 750.8 |
| stage/render/prepare_views | 0.136 | 0.173 | 1.34 | 685.9 |
| controller/move/ms | 0.135 | 0.255 | 0.62 | 685.8 |
| stage/main/last | 0.123 | 0.195 | 1.61 | 623.6 |
| stage/render/phase_sort | 0.099 | 0.150 | 0.37 | 501.4 |
| render/acquire/ms | 0.094 | 0.123 | 1.26 | 474.8 |
| physics/step/ms | 0.056 | 0.090 | 0.42 | 285.5 |
| pipelines/compile_ms | 0.056 | 0.000 | 118.83 | 285.3 |
| pipelines/front_ms | 0.054 | 0.000 | 114.76 | 274.0 |
| stage/render/prepare_collect_phase_buffers | 0.050 | 0.080 | 0.29 | 252.5 |
| physics/insert/ms | 0.043 | 0.412 | 0.92 | 215.7 |
| stage/render/cleanup | 0.034 | 0.110 | 0.65 | 173.4 |
| encode/resolve_diagnostics/sum | 0.032 | 0.045 | 0.88 | 163.0 |
| weather/map/ms | 0.026 | 0.034 | 0.10 | 132.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.23 | 75.9 |
| anim/apply/ms | 0.011 | 0.021 | 0.15 | 57.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.22 | 57.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.63 | 55.5 |
| controller/avoid/ms | 0.010 | 0.013 | 0.11 | 52.6 |
| script/update/ms | 0.007 | 0.009 | 0.14 | 34.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.15 | 30.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.6 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 26.4 |
| script/input/ms | 0.005 | 0.006 | 0.18 | 24.4 |
| controller/write/ms | 0.004 | 0.004 | 0.11 | 17.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 15.0 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.07 | 11.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.19 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 3.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 42 | 12 |
| crowd/c0/joints/count | count | 1272 | 2218 | 546 |
| crowd/c0/meshes/count | count | 349 | 633 | 132 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5539 | 6037 | 5650 |
| crowd/c1/meshes/count | count | 1177 | 1311 | 1209 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4283 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4253 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.65 | 6.86 | 2.45 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.23 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.63 | 0.02 |
| encode/finish_critical | ms | 2.65 | 6.86 | 2.45 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.88 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.79 | 31.67 | 10.09 |
| gpu/timed/ms | ms | 8.10 | 11.81 | 8.53 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 801 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 801 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 20 | 20 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3375 | 5102 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3375 | 5102 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 34 | 226 | 27 |
| phases/shadow/bins | count | 4662 | 5922 | 2057 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.3 k | 12.8 k | 5350 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 248 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 211 | 476 | 19 |
| physics/pending | count | 0 | 229 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 298 | 299 | 299 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 611 | 11 |
| preprocess/bind_groups_created | count | 11 | 611 | 11 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 176 | 288 | 166 |
| process/cpu_usage | % | 25.48 | 45.67 | 33.17 |
| process/mem_usage | GiB | 5.45 | 5.62 | 5.62 |
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
| started_load_count |  | 4917 | 5008 | 5008 |
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
| stream/latency/s | s | 0.04 | 0.15 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.45 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.73 | 78.03 | 35.14 |
| system/mem_usage | % | 69.24 | 71.62 | 69.61 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 85 | 273 | 77 |
| views/all/bins | count | 7214 | 8806 | 5555 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 25.2 k | 13.0 k |
| views/all/pipelines | count | 246 | 295 | 73 |
| views/all/pipelines_distinct | count | 70 | 73 | 61 |
| views/all/work_items | count | 17.0 k | 22.8 k | 13.3 k |
| views/cascade/batch_sets | count | 32 | 46 | 27 |
| views/cascade/bins | count | 2436 | 2988 | 2057 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7773 | 9715 | 5350 |
| views/cascade/pipelines | count | 32 | 46 | 27 |
| views/cascade/work_items | count | 7773 | 9715 | 5350 |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 2211 | 2962 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2430 | 3257 | 0 |
| views/lamp_face/pipelines | count | 168 | 208 | 0 |
| views/lamp_face/work_items | count | 0 | 2953 | 0 |
| views/main/batch_sets | count | 49 | 52 | 50 |
| views/main/bins | count | 2888 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8350 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 48 | 46 |
| views/main/work_items | count | 8520 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 62.83 | 323 | 318.0 k |
| anim/events/count | count | 0.11 | 3 | 550 |
| assets/Image/added | count | 0.12 | 28 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.80 | 152 | 4061 |
| assets/Mesh/removed | count | 0.96 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 29 | 464 |
| assets/StandardMaterial/modified | count | 4.28 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.92 | 1566 | 4676 |
| encode//count | count | 41.58 | 272 | 210.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5061 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5061 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5061 |
| physics/inserts | count | 0.44 | 110 | 2243 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.46 | 145 | 2335 |
| physics/steps | count | 0.78 | 2 | 3939 |
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
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6689 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1466 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.2 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.70 | 2695.9 | main/busy/ms 2668, stage/main/post_update 2565, stage/main/update 95.57, surface/advance_cpu_ms 67.18, stage/main/pre_update 3.06, stage/main/last 1.03, surface/cell_upload_ms 0.78, stage/main/first 0.63, stage/main/fixed_main_loop 0.57, stream/drive/ms 0.29, weather/map/ms 0.11, bigspace/total/ms 0.11 |
| warm-up | 3.13 | 430.8 | main/busy/ms 2796, stage/main/post_update 2597, render/thread/ms 399, render/busy/ms 303, stage/render/prepare_assets 277, stage/main/update 186, stage/render/prepare_views 93.38, render/acquire/ms 93.17, surface/advance_cpu_ms 67.18, stage/render/render 9.82, stage/render/create_views 7.88, encode//sum 6.68 |
| warm-up | 3.19 | 61.3 | render/thread/ms 442, render/busy/ms 345, stage/render/prepare_assets 302, main/busy/ms 146, stage/main/update 95.48, stage/render/prepare_views 94.21, render/acquire/ms 93.87, pipelines/compile_ms 79.83, pipelines/front_ms 76.21, stage/main/post_update 41.14, terrain/update/ms 24.02, stage/render/render 19.03 |
| warm-up | 3.37 | 149.1 | pipelines/wait_ms 3381, pipelines/compile_ms 1450, pipelines/front_ms 1370, render/thread/ms 169, render/busy/ms 160, main/busy/ms 83.17, pipelines/driver_ms 80.38, stage/render/prepare_write_phase_buffers 50.54, stage/main/post_update 49.58, stage/render/render 45.31, render/graph/ms 37.62, encode//sum 35.93 |
| warm-up | 3.43 | 59.5 | pipelines/wait_ms 5160, pipelines/compile_ms 1846, pipelines/front_ms 1750, render/thread/ms 192, render/busy/ms 190, main/busy/ms 139, pipelines/driver_ms 95.37, stage/main/update 67.30, stage/render/prepare_write_phase_buffers 50.27, encode//sum 46.93, stage/main/post_update 42.28, stage/render/render 39.89 |
| warm-up | 3.55 | 127.0 | pipelines/wait_ms 2036, pipelines/compile_ms 1788, pipelines/front_ms 1703, render/thread/ms 142, render/busy/ms 111, main/busy/ms 101, pipelines/driver_ms 84.13, stage/render/prepare_assets 74.82, stage/main/update 67.58, stage/render/render 47.09, render/present/ms 31.12, stage/main/post_update 18.46 |
| warm-up | 3.59 | 34.4 | pipelines/compile_ms 2293, pipelines/front_ms 2155, pipelines/wait_ms 426, pipelines/driver_ms 138, render/thread/ms 123, render/busy/ms 91.44, stage/render/prepare_assets 66.98, stage/render/render 49.18, render/present/ms 31.00, main/busy/ms 28.12, render/graph/ms 17.59, encode//sum 16.08 |
| warm-up | 3.75 | 79.5 | pipelines/compile_ms 880, pipelines/front_ms 852, main/busy/ms 54.10, render/thread/ms 51.33, render/busy/ms 49.15, stage/main/update 42.66, stream/drive/ms 38.42, stage/render/prepare_assets 33.39, pipelines/driver_ms 27.40, stage/render/render 11.21, encode//sum 9.28, render/graph/ms 8.83 |
| warm-up | 3.82 | 51.4 | pipelines/compile_ms 332, pipelines/front_ms 322, render/thread/ms 54.07, render/busy/ms 51.46, main/busy/ms 49.09, stage/main/update 32.50, stream/drive/ms 26.19, stage/render/render 22.79, stage/render/prepare_assets 21.88, render/graph/ms 19.84, stage/render/graph_submit 10.65, pipelines/driver_ms 10.20 |
| warm-up | 3.89 | 65.2 | pipelines/compile_ms 576, pipelines/front_ms 552, render/thread/ms 81.61, render/busy/ms 79.88, stage/render/prepare_assets 39.12, main/busy/ms 35.55, stage/render/render 33.22, render/graph/ms 30.96, pipelines/wait_ms 24.14, pipelines/driver_ms 23.58, pipelines/lock_ms 23.07, stage/render/graph_submit 19.02 |
| warm-up | 3.96 | 68.3 | pipelines/compile_ms 575, pipelines/front_ms 551, render/thread/ms 96.67, render/busy/ms 94.65, main/busy/ms 57.77, stage/render/prepare_assets 57.02, stage/main/update 37.18, stage/render/render 29.06, render/graph/ms 26.57, pipelines/driver_ms 23.73, pipelines/wait_ms 22.33, pipelines/lock_ms 21.42 |
| warm-up | 4.03 | 68.6 | pipelines/compile_ms 1397, pipelines/front_ms 1361, render/thread/ms 92.14, render/busy/ms 90.11, stage/render/prepare_assets 69.47, main/busy/ms 51.60, pipelines/driver_ms 36.34, stage/main/update 34.21, pipelines/wait_ms 12.09, stage/render/render 11.05, pipelines/lock_ms 10.77, stage/main/post_update 9.63 |
| warm-up | 4.06 | 34.1 | pipelines/compile_ms 1191, pipelines/front_ms 1159, render/thread/ms 75.84, render/busy/ms 71.24, main/busy/ms 45.61, stage/render/prepare_assets 38.16, pipelines/driver_ms 31.72, stage/render/render 24.52, stage/main/update 23.97, render/graph/ms 19.46, stage/render/camera_driver 11.24, stage/main/post_update 8.95 |
| warm-up | 4.11 | 44.6 | pipelines/compile_ms 515, pipelines/front_ms 500, render/thread/ms 61.50, render/busy/ms 56.85, main/busy/ms 50.75, stage/render/render 24.26, stage/main/update 22.92, stage/render/prepare_assets 21.41, render/graph/ms 19.33, pipelines/wait_ms 14.76, pipelines/driver_ms 14.52, stage/main/post_update 11.67 |
| warm-up | 4.15 | 41.5 | pipelines/compile_ms 457, pipelines/front_ms 446, render/thread/ms 71.14, render/busy/ms 69.26, main/busy/ms 48.72, stage/main/update 21.13, stage/render/prepare_assets 19.22, stage/main/post_update 18.10, stage/render/render 13.72, pipelines/wait_ms 13.20, pipelines/driver_ms 11.79, render/graph/ms 11.59 |
| warm-up | 4.23 | 83.2 | pipelines/compile_ms 1687, pipelines/front_ms 1642, pipelines/wait_ms 121, render/thread/ms 111, render/busy/ms 109, main/busy/ms 70.23, pipelines/driver_ms 44.84, stage/main/update 35.54, stage/main/post_update 28.91, stage/render/prepare_assets 28.38, stage/render/prepare_flush 17.16, stage/render/prepare_resources 16.89 |
| warm-up | 4.33 | 98.6 | pipelines/compile_ms 3117, pipelines/front_ms 3034, pipelines/wait_ms 839, render/thread/ms 163, render/busy/ms 160, main/busy/ms 83.02, pipelines/driver_ms 82.91, stage/render/prepare_assets 61.06, encode//sum 52.79, stage/main/post_update 46.25, stage/render/prepare_resources 33.58, stage/main/update 26.38 |
| warm-up | 4.42 | 90.2 | pipelines/compile_ms 2085, pipelines/front_ms 2026, pipelines/wait_ms 901, render/thread/ms 175, render/busy/ms 171, main/busy/ms 84.36, encode//sum 83.77, pipelines/driver_ms 58.41, stage/render/render 51.43, render/graph/ms 43.13, stage/main/post_update 42.65, stage/render/prepare_assets 39.55 |
| warm-up | 4.49 | 76.3 | pipelines/compile_ms 2069, pipelines/front_ms 2007, pipelines/wait_ms 767, render/thread/ms 141, render/busy/ms 138, main/busy/ms 69.21, pipelines/driver_ms 62.21, encode//sum 55.10, stage/render/render 46.21, render/graph/ms 37.99, stage/render/prepare_assets 29.13, stage/main/update 26.49 |
| warm-up | 4.55 | 52.3 | pipelines/compile_ms 2295, pipelines/front_ms 2234, pipelines/wait_ms 797, render/thread/ms 90.65, render/busy/ms 88.81, main/busy/ms 84.15, pipelines/driver_ms 60.79, stage/main/update 54.40, stream/drive/ms 48.22, stage/render/prepare_assets 43.05, encode//sum 23.17, stage/render/render 19.19 |
| warm-up | 4.60 | 52.4 | pipelines/compile_ms 1336, pipelines/front_ms 1305, pipelines/wait_ms 242, main/busy/ms 106, stage/main/update 85.53, render/thread/ms 54.59, render/busy/ms 52.25, stream/drive/ms 47.97, pipelines/driver_ms 30.84, stage/render/prepare_assets 23.43, stage/render/render 17.72, encode//sum 14.78 |
| warm-up | 4.72 | 121.1 | pipelines/compile_ms 987, pipelines/front_ms 959, main/busy/ms 134, stage/main/update 110, pipelines/wait_ms 107, render/thread/ms 97.61, render/busy/ms 95.03, stream/drive/ms 48.81, stage/render/render 44.45, pipelines/lock_ms 42.13, render/graph/ms 41.15, stage/render/graph_submit 33.33 |
| warm-up | 4.81 | 87.5 | pipelines/compile_ms 2017, pipelines/front_ms 1951, render/thread/ms 137, render/busy/ms 135, main/busy/ms 114, pipelines/wait_ms 106, stage/main/update 91.64, pipelines/lock_ms 68.51, pipelines/driver_ms 66.58, stage/render/prepare_assets 55.85, stage/render/render 51.85, stream/drive/ms 49.61 |
| warm-up | 4.85 | 40.0 | pipelines/compile_ms 1665, pipelines/front_ms 1607, main/busy/ms 105, render/thread/ms 91.99, render/busy/ms 90.17, stage/main/update 86.92, pipelines/driver_ms 58.05, stream/drive/ms 56.36, stage/render/prepare_assets 43.48, pipelines/wait_ms 29.81, pipelines/lock_ms 29.34, stage/render/render 25.29 |
| warm-up | 4.94 | 95.0 | pipelines/compile_ms 623, pipelines/front_ms 600, main/busy/ms 93.61, stage/main/update 75.02, render/thread/ms 63.08, render/busy/ms 61.16, stream/drive/ms 56.29, stage/render/prepare_assets 28.29, pipelines/driver_ms 23.69, encode//sum 16.86, stage/render/render 14.39, render/graph/ms 11.97 |
| warm-up | 4.98 | 37.4 | pipelines/compile_ms 1231, pipelines/front_ms 1191, main/busy/ms 70.45, render/thread/ms 67.17, render/busy/ms 65.21, stage/main/update 41.84, pipelines/driver_ms 40.35, stream/drive/ms 31.42, stage/render/prepare_assets 28.62, encode//sum 20.39, stage/render/render 15.27, render/graph/ms 12.66 |
| warm-up | 5.02 | 42.3 | pipelines/compile_ms 1135, pipelines/front_ms 1101, main/busy/ms 69.83, render/thread/ms 55.33, render/busy/ms 53.40, stage/main/update 41.32, pipelines/driver_ms 33.64, stream/drive/ms 30.93, encode//sum 19.95, stage/render/render 17.32, stage/render/prepare_assets 15.93, pipelines/wait_ms 15.63 |
| warm-up | 5.06 | 35.2 | pipelines/compile_ms 804, pipelines/front_ms 777, render/thread/ms 54.28, render/busy/ms 52.43, main/busy/ms 41.46, pipelines/driver_ms 27.16, stage/main/update 18.30, encode//sum 17.70, stage/render/render 17.17, stage/main/post_update 15.57, render/graph/ms 14.82, pipelines/wait_ms 12.45 |
| warm-up | 5.09 | 31.8 | pipelines/compile_ms 455, pipelines/front_ms 438, render/thread/ms 58.86, render/busy/ms 57.21, main/busy/ms 39.90, encode//sum 18.78, stage/main/update 18.72, stage/render/prepare_assets 18.71, pipelines/driver_ms 17.15, stage/render/render 15.48, stage/main/post_update 13.86, render/graph/ms 13.15 |
| warm-up | 5.12 | 32.1 | render/thread/ms 54.81, render/busy/ms 53.14, main/busy/ms 37.11, encode//sum 19.83, stage/main/post_update 16.19, stage/render/render 15.38, stage/main/update 15.16, stage/render/prepare_assets 14.75, render/graph/ms 13.07, stage/render/graph_submit 9.81, stage/render/prepare_write_phase_buffers 7.90, stage/render/prepare_resources 7.87 |
| warm-up | 5.17 | 46.8 | pipelines/compile_ms 606, pipelines/front_ms 592, render/thread/ms 71.70, render/busy/ms 69.67, main/busy/ms 42.25, encode//sum 21.34, stage/render/prepare_assets 19.89, stage/render/render 18.72, stage/main/post_update 18.62, stage/main/update 17.44, render/graph/ms 15.99, pipelines/driver_ms 14.24 |
| warm-up | 5.20 | 33.4 | pipelines/compile_ms 780, pipelines/front_ms 760, render/thread/ms 72.66, render/busy/ms 70.47, main/busy/ms 38.89, stage/render/render 21.86, stage/render/prepare_assets 20.28, pipelines/driver_ms 20.04, encode//sum 19.87, render/graph/ms 19.02, stage/main/update 17.96, stage/render/graph_submit 15.28 |
| warm-up | 5.25 | 52.2 | pipelines/compile_ms 827, pipelines/front_ms 801, render/thread/ms 69.12, render/busy/ms 66.95, main/busy/ms 39.76, stage/render/prepare_assets 27.88, pipelines/driver_ms 26.38, stage/main/update 21.64, stage/render/render 19.33, encode//sum 17.79, render/graph/ms 16.65, stage/render/graph_submit 13.64 |
| warm-up | 5.30 | 40.9 | pipelines/compile_ms 938, pipelines/front_ms 909, render/thread/ms 75.05, render/busy/ms 73.12, main/busy/ms 46.57, stage/render/prepare_assets 35.30, pipelines/driver_ms 29.04, stage/render/render 19.58, stage/main/update 18.68, encode//sum 18.43, render/graph/ms 17.10, stage/render/graph_submit 13.71 |
| warm-up | 5.33 | 38.9 | pipelines/compile_ms 321, pipelines/front_ms 311, render/thread/ms 73.51, render/busy/ms 71.75, main/busy/ms 51.77, stage/main/post_update 21.67, stage/render/render 19.85, encode//sum 19.69, stage/render/prepare_resources 19.27, render/graph/ms 17.60, stage/render/graph_submit 13.98, stage/render/prepare_assets 12.95 |
| warm-up | 5.37 | 39.8 | pipelines/compile_ms 255, pipelines/front_ms 248, render/thread/ms 73.94, render/busy/ms 72.16, main/busy/ms 49.93, stage/main/post_update 30.99, stage/render/prepare_resources 24.62, encode//sum 20.13, stage/render/render 17.79, render/graph/ms 15.25, stage/main/update 13.29, stage/render/graph_submit 11.91 |
| warm-up | 5.44 | 32.4 | pipelines/compile_ms 914, pipelines/front_ms 892, render/thread/ms 53.02, render/busy/ms 51.59, stage/render/prepare_assets 29.53, main/busy/ms 26.25, pipelines/driver_ms 21.56, encode//sum 17.71, stage/render/render 12.56, stage/main/update 11.30, render/graph/ms 10.59, stage/main/post_update 9.04 |
| warm-up | 5.55 | 45.2 | render/thread/ms 52.00, render/busy/ms 50.09, main/busy/ms 37.26, encode//sum 25.51, stage/render/prepare_assets 24.05, stage/main/update 20.49, stage/render/render 18.00, render/graph/ms 15.38, stage/render/graph_submit 12.74, stage/main/post_update 11.76, stream/drive/ms 10.83, encode/submit 6.91 |
| warm-up | 5.61 | 33.2 | pipelines/compile_ms 317, pipelines/front_ms 308, render/thread/ms 53.76, render/busy/ms 51.92, main/busy/ms 23.70, stage/render/prepare_assets 22.38, encode//sum 21.12, stage/render/render 15.92, render/graph/ms 13.47, stage/main/post_update 10.87, stage/render/graph_submit 10.67, pipelines/driver_ms 9.40 |
| warm-up | 6.21 | 33.4 | render/thread/ms 42.69, render/busy/ms 41.39, main/busy/ms 28.60, encode//sum 19.69, stage/render/prepare_assets 14.06, stage/render/render 12.61, stage/main/update 11.22, stage/main/post_update 11.09, render/graph/ms 10.90, stage/render/graph_submit 8.78, encode/finish_wall 4.92, stage/render/specialize 4.56 |
| warm-up | 6.46 | 32.1 | pipelines/compile_ms 83.26, pipelines/front_ms 77.23, render/thread/ms 50.57, render/busy/ms 49.33, main/busy/ms 44.46, encode//sum 25.69, stage/main/post_update 21.00, stage/main/update 16.97, stage/render/render 14.97, stage/render/prepare_assets 14.12, render/graph/ms 12.88, stage/render/graph_submit 8.59 |
| warm-up | 6.56 | 34.3 | main/busy/ms 45.54, render/thread/ms 39.96, render/busy/ms 38.31, stage/main/update 25.72, encode//sum 23.82, stage/render/render 17.50, stream/drive/ms 15.39, render/graph/ms 15.00, stage/main/post_update 14.16, stage/render/graph_submit 12.31, stage/render/prepare_assets 6.88, encode/finish_wall 6.32 |
| warm-up | 6.99 | 32.3 | main/busy/ms 40.65, render/thread/ms 28.04, render/busy/ms 26.72, encode//sum 24.43, stage/main/update 21.27, stage/render/render 14.98, stream/drive/ms 14.59, stream/spawn/ms 13.39, render/graph/ms 13.22, stage/main/post_update 12.97, stage/render/graph_submit 10.34, encode/finish_wall 5.98 |
| measure | 18.19 | 31.2 | main/busy/ms 40.38, encode//sum 28.87, render/thread/ms 25.65, stage/main/update 25.16, render/busy/ms 24.46, stream/drive/ms 21.52, stage/render/render 15.98, render/graph/ms 14.32, stage/main/post_update 12.10, stage/render/graph_submit 11.46, encode/finish_wall 6.12, encode/submit 5.42 |
| measure | 23.27 | 34.6 | render/thread/ms 57.64, render/busy/ms 55.50, encode//sum 51.07, main/busy/ms 46.19, stage/render/render 34.75, render/graph/ms 31.98, stage/main/post_update 27.25, stage/render/graph_submit 25.52, encode/finish_wall 14.36, stage/main/update 14.10, encode/submit 11.41, stage/render/camera_driver 6.11 |
| measure | 23.31 | 36.8 | render/thread/ms 63.10, render/busy/ms 61.06, encode//sum 54.40, main/busy/ms 47.54, stage/render/render 32.94, render/graph/ms 30.04, stage/main/post_update 29.55, stage/render/graph_submit 23.00, encode/finish_wall 14.19, stage/main/update 12.36, stage/render/prepare_assets 11.35, encode/submit 8.95 |
| measure | 23.34 | 33.7 | encode//sum 59.88, render/thread/ms 57.77, render/busy/ms 56.27, main/busy/ms 42.60, stage/render/render 27.41, stage/main/post_update 26.46, render/graph/ms 25.04, stage/render/graph_submit 19.02, stage/render/prepare_assets 14.65, encode/finish_wall 12.24, stage/main/update 9.66, encode/submit 6.90 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_off\balmora_walk@2560x1440_off=cascadeencoders_cascadehold_meshslabs-3 --bench-baseline none --off cascadeencoders,cascadehold,meshslabs
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 4%
- during the run: other processes 12% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **73%** (min 41%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.1 W · 71 °C (max 76) · VRAM 5984 MiB · throttled 100% of samples
