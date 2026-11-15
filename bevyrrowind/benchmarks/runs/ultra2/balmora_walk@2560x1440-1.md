# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4840 frames over 65.7 s · **median 12.61 ms (79 fps)** · mean 13.56 · p95 18.92 · p99 25.90 (1% low 39 fps) · worst 121.04 ms · 242 spikes over 1.5× median · 71.3 stutters/min (work 106.0/min)
- warm-up: 337 frames over 8.7 s, worst 2535.8 ms
- hitches (> 31.5 ms): **18** measured, 20 in warm-up
- timed GPU passes: 8.38 ms of a 13.56 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.61 ms, limit 11 ms
- **FAIL** frame_ms: 78 stutters (> 1.5x moving median of 60) in 1.09 min = 71.28/min, limit 1/min
- **FAIL** worst frame 121.04 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.984 | 2.091 | 2.289 | 0.95 | 0.183 |
| main_opaque_pass_3d | 1.692 | 1.783 | 2.288 | 0.95 | 0.058 |
| auxiliary_views | 1.116 | 1.176 | 8.403 | 0.95 | 0.155 |
| early prepass | 0.499 | 0.526 | 0.735 | 0.95 | 0.079 |
| ssao | 0.316 | 0.333 | 0.332 | 0.95 | 0.008 |
| bloom | 0.307 | 0.323 | 0.327 | 0.95 | 0.065 |
| atmosphere_luts | 0.301 | 0.317 | 0.377 | 0.95 | 0.003 |
| water_velocity | 0.268 | 0.282 | 0.354 | 0.95 | 0.001 |
| water_sspr | 0.208 | 0.219 | 0.219 | 0.95 | 0.002 |
| uniform_allocation | 0.194 | 0.205 | 0.215 | 0.95 | 0.035 |
| water_fft | 0.164 | 0.172 | 0.176 | 0.95 | 0.003 |
| bin_unpacking | 0.159 | 0.168 | 0.397 | 0.95 | 0.025 |
| atmos_froxel_inject | 0.147 | 0.155 | 0.156 | 0.95 | 0.001 |
| clouds_trace | 0.135 | 0.142 | 0.232 | 0.95 | 0.001 |
| upscaling | 0.115 | 0.122 | 0.130 | 0.95 | 0.004 |
| clouds_composite | 0.103 | 0.109 | 0.111 | 0.95 | 0.002 |
| tonemapping | 0.083 | 0.088 | 0.091 | 0.95 | 0.004 |
| main_transmissive_pass_3d | 0.083 | 0.087 | 0.169 | 0.95 | 0.008 |
| clustering | 0.064 | 0.068 | 0.071 | 0.95 | 0.020 |
| early_mesh_preprocessing | 0.051 | 0.053 | 0.056 | 0.95 | 0.005 |
| render_sky | 0.051 | 0.053 | 0.054 | 0.95 | 0.002 |
| clouds_resolve | 0.047 | 0.049 | 0.050 | 0.95 | 0.001 |
| surface_trail | 0.046 | 0.058 | 0.058 | 0.79 | 0.002 |
| lightprobe_irradiance_map | 0.045 | 0.436 | 0.616 | 0.10 | 0.001 |
| water_snapshot | 0.032 | 0.033 | 0.036 | 0.95 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.95 | 0.003 |
| per_view_shadow_pass | 0.026 | 0.027 | 0.030 | 0.95 | 0.007 |
| atmos_froxel_integrate | 0.025 | 0.027 | 0.029 | 0.95 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.226 | 0.234 | 0.10 | 0.004 |
| water_mips | 0.022 | 0.023 | 0.023 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.032 | 0.95 | 0.109 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.95 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.95 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.95 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 23.336 | 0.000 | 10878.62 | 112946.5 |
| render/thread/ms | 12.068 | 17.135 | 119.28 | 58409.3 |
| encode//sum | 11.941 | 18.767 | 148.79 | 57792.3 |
| render/busy/ms | 11.124 | 14.679 | 82.86 | 53842.1 |
| main/busy/ms | 9.793 | 13.133 | 27.28 | 47400.1 |
| render/graph/ms | 6.750 | 9.064 | 23.81 | 32671.5 |
| stage/main/post_update | 6.236 | 9.051 | 23.12 | 30180.1 |
| stage/render/graph_submit | 5.517 | 7.410 | 22.78 | 26703.8 |
| stage/render/render | 4.595 | 9.394 | 22.71 | 22238.1 |
| encode/finish_wall | 3.158 | 4.327 | 16.86 | 15283.9 |
| stage/render/cleanup | 3.015 | 9.380 | 24.56 | 14594.6 |
| encode/submit | 2.449 | 3.653 | 20.21 | 11854.0 |
| stage/main/update | 2.204 | 2.798 | 19.36 | 10668.2 |
| stage/render/camera_driver | 1.172 | 1.674 | 5.01 | 5670.2 |
| stage/render/prepare_write_phase_buffers | 1.058 | 1.452 | 3.20 | 5120.0 |
| stage/render/prepare_assets | 0.597 | 0.791 | 10.45 | 2890.1 |
| render/present/ms | 0.525 | 0.773 | 2.89 | 2538.8 |
| bigspace/total/ms | 0.520 | 1.053 | 4.06 | 2515.9 |
| pipelines/compile_ms | 0.485 | 0.000 | 185.33 | 2346.8 |
| stage/render/prepare_views | 0.463 | 2.043 | 97.15 | 2240.7 |
| stage/render/prepare_resources | 0.462 | 0.598 | 3.41 | 2236.5 |
| pipelines/front_ms | 0.462 | 0.000 | 177.82 | 2236.0 |
| stage/main/first | 0.443 | 0.626 | 5.49 | 2145.4 |
| render/acquire/ms | 0.419 | 1.995 | 97.09 | 2028.4 |
| stage/render/prepare_meshes | 0.410 | 0.587 | 2.46 | 1984.2 |
| stage/render/queue | 0.347 | 0.665 | 5.40 | 1679.6 |
| stage/render/specialize | 0.344 | 0.578 | 3.38 | 1663.4 |
| stage/main/pre_update | 0.339 | 0.507 | 1.52 | 1642.0 |
| terrain/update/ms | 0.301 | 0.386 | 1.97 | 1457.9 |
| stage/render/prepare_bind_groups | 0.279 | 0.517 | 3.35 | 1351.5 |
| terrain/select/ms | 0.268 | 0.342 | 0.92 | 1295.5 |
| stage/main/fixed_main_loop | 0.268 | 0.436 | 1.39 | 1295.4 |
| controller/feed/ms | 0.268 | 0.365 | 0.79 | 1295.0 |
| stream/drive/ms | 0.259 | 0.376 | 17.36 | 1254.6 |
| stage/render/create_views | 0.244 | 0.268 | 53.67 | 1182.3 |
| bigspace/low_precision/ms | 0.218 | 0.541 | 3.39 | 1055.1 |
| stream/tick/ms | 0.197 | 0.277 | 0.74 | 954.5 |
| far/select/ms | 0.197 | 0.276 | 0.66 | 951.9 |
| stage/render/prepare_batch_phases | 0.182 | 0.260 | 0.86 | 881.4 |
| stage/render/prepare_flush | 0.165 | 0.332 | 0.93 | 797.5 |
| anim/evaluate_cpu/ms | 0.162 | 0.210 | 0.82 | 784.4 |
| stage/render/extract_commands | 0.160 | 0.202 | 0.50 | 776.3 |
| controller/move/ms | 0.159 | 0.434 | 0.88 | 770.1 |
| bigspace/high_precision/ms | 0.140 | 0.300 | 3.65 | 679.8 |
| anim/evaluate/ms | 0.125 | 0.250 | 2.57 | 602.9 |
| stage/main/last | 0.119 | 0.217 | 1.59 | 575.9 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.133 | 0.64 | 298.7 |
| physics/step/ms | 0.060 | 0.101 | 0.74 | 292.4 |
| physics/insert/ms | 0.055 | 0.439 | 1.86 | 266.6 |
| encode/resolve_diagnostics/sum | 0.034 | 0.052 | 0.80 | 165.4 |
| stage/render/phase_sort | 0.031 | 0.053 | 0.38 | 149.5 |
| weather/map/ms | 0.025 | 0.034 | 0.15 | 119.1 |
| pipelines/driver_ms | 0.023 | 0.000 | 28.36 | 110.8 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.026 | 0.60 | 78.0 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.38 | 58.6 |
| anim/apply/ms | 0.011 | 0.020 | 0.10 | 55.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 51.0 |
| controller/avoid/ms | 0.010 | 0.014 | 0.11 | 50.0 |
| script/update/ms | 0.007 | 0.010 | 0.19 | 35.7 |
| stream/despawn/ms | 0.006 | 0.000 | 0.93 | 29.0 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 25.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 25.5 |
| script/ui/ms | 0.005 | 0.007 | 0.14 | 25.0 |
| controller/write/ms | 0.004 | 0.005 | 0.06 | 17.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.60 | 14.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 4.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 12 | 0 |
| anim/a2/count | count | 1 | 14 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 50 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1272 | 2262 | 661 |
| crowd/c0/meshes/count | count | 349 | 646 | 173 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5983 | 5535 |
| crowd/c1/meshes/count | count | 1180 | 1297 | 1168 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 242 | 286 | 273 |
| crowd/joints_extracted/count | count | 5316 | 16.5 k | 7309 |
| crowd/joints_uploaded/count | count | 5284 | 7784 | 7309 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.60 | 16.38 | 1.98 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.60 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.38 | 0.01 |
| encode/finish_critical | ms | 2.60 | 16.38 | 1.98 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.80 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.76 | 82.86 | 9.24 |
| gpu/timed/ms | ms | 8.57 | 32.89 | 8.53 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 552.74 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 796 | 1416 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 796 | 1416 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 22 | 20 |
| phases/opaque3d/bins | count | 1295 | 1396 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3446 | 5291 | 3202 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 21 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1295 | 1396 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3446 | 5291 | 3202 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 47 | 331 | 39 |
| phases/shadow/bins | count | 5810 | 8247 | 3217 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 14.0 k | 18.2 k | 8633 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 215 | 364 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 1217 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 185 | 187 | 187 |
| pipelines/waiting | count | 0 | 25 | 0 |
| preprocess/bind_groups | count | 13 | 848 | 12 |
| preprocess/bind_groups_created | count | 13 | 848 | 12 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 203 | 318 | 190 |
| process/cpu_usage | % | 25.36 | 51.13 | 25.24 |
| process/mem_usage | GiB | 5.96 | 6.18 | 6.18 |
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
| started_load_count |  | 4769 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 432 | 528 | 396 |
| statics/materials_bound | count | 432 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.13 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.40 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.94 | 63.25 | 29.01 |
| system/mem_usage | % | 67.40 | 69.05 | 68.88 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2841 | 2952 | 2952 |
| textures/images | count | 2592 | 2645 | 2645 |
| views/all/batch_sets | count | 97 | 379 | 89 |
| views/all/bins | count | 8458 | 11.1 k | 6715 |
| views/all/count | count | 47 | 79 | 5 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 22.9 k | 31.0 k | 16.3 k |
| views/all/pipelines | count | 257 | 412 | 85 |
| views/all/pipelines_distinct | count | 69 | 75 | 61 |
| views/all/work_items | count | 20.6 k | 30.0 k | 16.6 k |
| views/cascade/batch_sets | count | 47 | 58 | 39 |
| views/cascade/bins | count | 3662 | 4363 | 3217 |
| views/cascade/count | count | 4 | 6 | 4 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 11.6 k | 13.9 k | 8633 |
| views/cascade/pipelines | count | 47 | 58 | 39 |
| views/cascade/work_items | count | 11.6 k | 13.9 k | 8633 |
| views/lamp_face/batch_sets | count | 0 | 283 | 0 |
| views/lamp_face/bins | count | 2149 | 3991 | 0 |
| views/lamp_face/count | count | 42 | 72 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2391 | 4435 | 0 |
| views/lamp_face/pipelines | count | 168 | 312 | 0 |
| views/lamp_face/work_items | count | 0 | 4254 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2894 | 3564 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8498 | 13.3 k | 7640 |
| views/main/pipelines | count | 46 | 50 | 46 |
| views/main/work_items | count | 8659 | 13.6 k | 7948 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.5 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 46.87 | 540 | 226.8 k |
| anim/events/count | count | 0.11 | 3 | 543 |
| assets/Image/added | count | 0.14 | 43 | 658 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.89 | 152 | 4320 |
| assets/Mesh/removed | count | 1.01 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 534 |
| assets/StandardMaterial/modified | count | 4.45 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.01 | 2465 | 9719 |
| encode//count | count | 40.22 | 392 | 194.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 4840 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 4840 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 4840 |
| physics/inserts | count | 0.71 | 170 | 3431 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 145 | 2335 |
| physics/steps | count | 0.81 | 3 | 3927 |
| physics/steps_dropped | count | 0.00 | 4 | 12 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 18 | 378 |
| stream/upload/bytes | bytes | 6994 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1533 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.5 ms; slowest 38, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2535.8 | main/busy/ms 2510, stage/main/post_update 2410, stage/main/update 94.22, surface/advance_cpu_ms 66.30, stage/main/pre_update 2.37, stage/main/last 0.91, surface/cell_upload_ms 0.74, stage/main/first 0.51, stage/main/fixed_main_loop 0.44, stream/drive/ms 0.37, stream/tick/ms 0.13, bigspace/total/ms 0.12 |
| warm-up | 3.24 | 709.1 | main/busy/ms 2545, stage/main/post_update 2436, render/thread/ms 672, render/busy/ms 514, stage/render/prepare_assets 437, stage/render/prepare_views 155, render/acquire/ms 155, stage/main/update 97.98, surface/advance_cpu_ms 66.30, stage/render/prepare_resources 48.76, stage/render/create_views 19.01, stage/render/render 8.79 |
| warm-up | 3.48 | 235.4 | pipelines/wait_ms 2577, render/thread/ms 686, render/busy/ms 527, stage/render/prepare_assets 438, pipelines/compile_ms 249, pipelines/front_ms 232, stage/render/prepare_views 155, render/acquire/ms 155, main/busy/ms 54.48, stage/render/prepare_resources 50.77, stage/main/post_update 35.43, encode//sum 23.14 |
| warm-up | 3.78 | 105.5 | pipelines/compile_ms 278, pipelines/front_ms 262, main/busy/ms 76.28, render/thread/ms 62.60, render/busy/ms 59.90, stage/main/update 59.79, stage/render/prepare_assets 45.52, pipelines/driver_ms 16.78, stage/main/post_update 12.21, stage/render/render 11.27, render/graph/ms 7.90, stage/render/graph_submit 5.32 |
| warm-up | 3.83 | 32.3 | render/thread/ms 41.26, render/busy/ms 39.46, main/busy/ms 29.23, stage/render/prepare_assets 27.71, stage/main/post_update 14.47, stage/main/update 9.42, stage/render/render 6.80, render/graph/ms 4.34, stream/drive/ms 2.90, stage/render/graph_submit 2.38, encode//sum 2.13, stage/render/camera_driver 1.83 |
| warm-up | 3.96 | 94.3 | pipelines/wait_ms 1170, pipelines/compile_ms 220, pipelines/front_ms 205, main/busy/ms 55.74, render/thread/ms 54.15, render/busy/ms 52.25, stage/main/update 42.56, stage/render/prepare_assets 39.01, stream/drive/ms 37.88, pipelines/driver_ms 14.79, stage/main/post_update 8.62, render/graph/ms 5.67 |
| warm-up | 4.08 | 89.7 | pipelines/wait_ms 4590, pipelines/compile_ms 126, pipelines/front_ms 116, main/busy/ms 80.95, render/thread/ms 65.52, render/busy/ms 62.05, stage/main/update 56.78, stream/drive/ms 44.94, stage/render/prepare_assets 32.76, stage/main/post_update 18.87, render/graph/ms 13.66, stage/render/render 10.56 |
| warm-up | 4.11 | 32.6 | pipelines/wait_ms 4590, pipelines/compile_ms 126, pipelines/front_ms 116, render/thread/ms 66.21, render/busy/ms 63.75, main/busy/ms 48.63, stage/render/prepare_assets 44.28, stage/main/update 24.35, stage/main/post_update 14.46, stream/drive/ms 13.52, pipelines/driver_ms 9.73, render/graph/ms 8.64 |
| warm-up | 4.17 | 59.5 | render/thread/ms 62.34, render/busy/ms 60.15, stage/render/prepare_assets 41.39, main/busy/ms 39.57, stage/main/update 16.90, stream/drive/ms 12.24, stage/main/post_update 11.29, stage/render/render 9.87, render/graph/ms 7.12, stage/main/fixed_main_loop 4.89, stage/render/graph_submit 4.73, stage/main/pre_update 4.38 |
| warm-up | 4.23 | 51.8 | render/thread/ms 76.13, render/busy/ms 73.95, stage/render/prepare_assets 54.41, main/busy/ms 54.29, stage/main/update 38.16, stream/drive/ms 33.12, stage/render/render 10.89, stage/main/post_update 8.36, render/graph/ms 8.09, stage/render/graph_submit 6.00, encode/submit 5.42, stage/main/fixed_main_loop 3.30 |
| warm-up | 4.32 | 97.1 | pipelines/wait_ms 5115, pipelines/compile_ms 238, pipelines/front_ms 224, render/thread/ms 110, render/busy/ms 77.76, stage/render/prepare_assets 65.27, main/busy/ms 51.23, stage/main/update 37.68, stage/render/cleanup 33.36, stream/drive/ms 32.87, render/present/ms 32.25, pipelines/driver_ms 14.54 |
| warm-up | 4.46 | 39.6 | render/thread/ms 55.88, render/busy/ms 52.88, main/busy/ms 30.18, encode//sum 17.94, stage/render/prepare_assets 16.94, stage/main/update 13.85, render/graph/ms 12.72, stage/main/post_update 10.97, stage/render/render 9.44, stream/drive/ms 8.02, stage/render/cleanup 7.51, stage/render/graph_submit 6.65 |
| warm-up | 4.52 | 65.5 | pipelines/wait_ms 1730, pipelines/compile_ms 215, pipelines/front_ms 200, render/thread/ms 82.05, main/busy/ms 80.56, render/busy/ms 78.27, stage/main/update 65.38, stream/drive/ms 60.41, encode//sum 51.78, stage/render/prepare_assets 27.05, stage/render/render 21.11, render/graph/ms 17.52 |
| warm-up | 4.57 | 52.1 | pipelines/wait_ms 1730, pipelines/compile_ms 215, pipelines/front_ms 200, main/busy/ms 79.68, render/thread/ms 77.38, render/busy/ms 74.72, stage/main/update 64.04, stream/drive/ms 60.20, encode//sum 44.87, stage/render/prepare_assets 31.60, stage/render/render 17.44, render/graph/ms 15.14 |
| warm-up | 4.67 | 71.3 | main/busy/ms 80.55, stage/main/update 59.98, stream/drive/ms 52.65, render/thread/ms 43.27, render/busy/ms 41.87, stage/render/prepare_assets 18.38, stage/main/post_update 13.77, stage/render/specialize 5.46, render/graph/ms 5.44, encode//sum 4.68, stage/render/cleanup 4.67, stage/render/render 3.83 |
| warm-up | 4.74 | 47.2 | pipelines/wait_ms 5497, pipelines/compile_ms 265, pipelines/front_ms 250, main/busy/ms 50.70, stage/main/update 40.56, stream/drive/ms 35.90, render/thread/ms 31.54, render/busy/ms 30.23, pipelines/driver_ms 14.56, stage/render/prepare_assets 13.72, stage/render/render 6.33, stage/main/post_update 5.69 |
| warm-up | 5.63 | 33.1 | pipelines/wait_ms 7335, pipelines/compile_ms 202, pipelines/front_ms 185, render/thread/ms 45.06, render/busy/ms 42.71, main/busy/ms 36.16, pipelines/driver_ms 16.59, stage/main/post_update 15.42, stage/render/render 13.58, stage/main/update 12.26, stage/render/prepare_assets 10.86, render/graph/ms 10.50 |
| warm-up | 5.67 | 35.5 | pipelines/wait_ms 5450, render/thread/ms 52.04, render/busy/ms 50.37, pipelines/compile_ms 42.17, pipelines/front_ms 39.89, main/busy/ms 34.72, stage/render/prepare_assets 22.33, stage/main/post_update 14.64, stage/main/update 11.27, stage/render/render 9.21, stage/render/specialize 7.79, render/graph/ms 7.00 |
| warm-up | 5.97 | 32.5 | main/busy/ms 41.18, render/thread/ms 32.27, render/busy/ms 30.59, stage/main/update 23.76, stream/drive/ms 17.80, stage/main/post_update 12.04, stage/render/render 9.40, encode//sum 7.64, render/graph/ms 7.15, stage/render/prepare_assets 6.83, stage/render/graph_submit 4.51, stage/render/prepare_write_phase_buffers 3.60 |
| warm-up | 8.23 | 53.9 | main/busy/ms 63.65, render/thread/ms 49.04, stage/main/update 48.31, render/busy/ms 47.05, encode//sum 24.32, stage/render/render 20.44, render/graph/ms 17.45, stage/render/graph_submit 14.28, stage/render/prepare_assets 13.98, stage/main/post_update 11.34, encode/finish_wall 7.72, encode/submit 6.73 |
| measure | 2.18 | 33.2 | encode//sum 70.96, render/thread/ms 44.66, render/busy/ms 43.05, render/graph/ms 22.82, main/busy/ms 19.34, stage/render/graph_submit 18.03, stage/render/cleanup 17.05, pipelines/compile_ms 12.69, pipelines/front_ms 12.46, stage/main/post_update 12.16, encode/finish_wall 9.51, encode/submit 8.61 |
| measure | 2.72 | 33.3 | encode//sum 63.24, render/thread/ms 43.77, render/busy/ms 42.51, stage/render/render 24.42, render/graph/ms 22.62, main/busy/ms 20.34, stage/render/graph_submit 17.62, stage/main/post_update 13.68, encode/submit 8.92, encode/finish_wall 8.75, stage/render/create_views 8.33, stage/render/camera_driver 4.87 |
| measure | 8.17 | 78.0 | render/thread/ms 86.28, render/busy/ms 85.17, stage/render/create_views 53.84, encode//sum 53.47, stage/render/cleanup 21.77, render/graph/ms 20.12, main/busy/ms 17.80, stage/render/graph_submit 15.39, stage/main/post_update 10.98, encode/submit 7.80, encode/finish_wall 7.65, stage/render/camera_driver 4.60 |
| measure | 9.34 | 72.9 | render/thread/ms 82.90, render/busy/ms 81.71, encode//sum 67.19, stage/render/create_views 43.94, stage/render/render 27.75, render/graph/ms 25.94, main/busy/ms 20.21, stage/render/graph_submit 20.04, stage/main/post_update 12.94, encode/submit 10.12, encode/finish_wall 9.97, stage/render/camera_driver 5.79 |
| measure | 9.42 | 62.3 | render/thread/ms 77.74, render/busy/ms 76.37, encode//sum 64.85, stage/render/create_views 32.88, stage/render/render 30.93, render/graph/ms 28.99, stage/render/graph_submit 22.81, main/busy/ms 21.55, encode/submit 13.62, stage/main/post_update 13.38, encode/finish_wall 9.27, stage/render/camera_driver 6.04 |
| measure | 12.03 | 75.5 | render/thread/ms 85.90, render/busy/ms 84.14, encode//sum 81.39, stage/render/create_views 41.95, stage/render/render 30.75, render/graph/ms 28.25, stage/render/graph_submit 21.85, main/busy/ms 19.26, stage/main/post_update 12.95, encode/submit 11.15, encode/finish_wall 10.77, pipelines/compile_ms 8.75 |
| measure | 12.12 | 71.9 | render/thread/ms 81.97, stage/render/prepare_views 51.89, render/acquire/ms 51.80, encode//sum 42.17, render/busy/ms 28.91, main/busy/ms 23.06, render/graph/ms 18.21, stage/render/graph_submit 15.28, stage/render/cleanup 15.24, stage/main/post_update 15.09, encode/submit 7.98, encode/finish_wall 7.38 |
| measure | 16.08 | 33.1 | render/thread/ms 58.36, stage/render/prepare_views 32.01, render/acquire/ms 31.83, encode//sum 30.75, render/busy/ms 25.42, main/busy/ms 19.35, stage/render/render 17.27, render/graph/ms 15.33, stage/render/graph_submit 12.77, stage/main/post_update 11.25, encode/submit 7.08, encode/finish_wall 5.80 |
| measure | 18.83 | 85.2 | render/thread/ms 99.67, render/busy/ms 96.92, encode//sum 81.99, stage/render/create_views 52.60, render/graph/ms 28.53, main/busy/ms 25.98, stage/render/graph_submit 22.83, stage/render/cleanup 21.38, stage/main/post_update 17.94, encode/submit 13.21, encode/finish_wall 11.10, stage/render/render 10.80 |
| measure | 18.87 | 40.4 | render/thread/ms 98.77, render/busy/ms 97.54, encode//sum 80.44, stage/render/create_views 52.70, stage/render/cleanup 44.47, render/graph/ms 26.50, main/busy/ms 25.60, stage/render/graph_submit 20.70, stage/main/post_update 15.93, encode/finish_wall 11.06, encode/submit 9.69, stage/render/render 7.08 |
| measure | 18.99 | 121.0 | pipelines/compile_ms 151, pipelines/front_ms 146, render/thread/ms 135, stage/render/prepare_views 97.32, render/acquire/ms 97.21, encode//sum 42.11, render/busy/ms 34.32, stage/render/render 24.05, main/busy/ms 23.32, stage/render/cleanup 23.14, render/graph/ms 20.50, stage/render/graph_submit 17.12 |
| measure | 20.87 | 33.2 | encode//sum 167, render/thread/ms 54.71, render/busy/ms 41.42, main/busy/ms 36.61, stage/render/cleanup 32.35, render/graph/ms 30.42, stage/main/post_update 28.77, stage/render/graph_submit 27.60, encode/finish_wall 20.14, stage/render/prepare_views 12.21, render/acquire/ms 12.09, encode/submit 7.56 |
| measure | 21.63 | 32.6 | render/thread/ms 47.16, encode//sum 33.48, render/busy/ms 29.64, main/busy/ms 22.94, render/graph/ms 18.81, stage/render/graph_submit 15.69, stage/main/post_update 15.56, stage/render/prepare_views 15.14, render/acquire/ms 15.03, stage/render/render 12.27, stage/render/cleanup 9.95, encode/submit 8.71 |
| measure | 23.25 | 38.2 | render/thread/ms 58.63, render/busy/ms 54.30, encode//sum 46.09, main/busy/ms 37.52, stage/render/render 24.38, render/graph/ms 20.41, stage/main/post_update 19.34, stage/render/graph_submit 16.67, stage/main/update 13.24, stage/render/prepare_assets 11.00, encode/finish_wall 9.46, encode/submit 8.86 |
| measure | 23.35 | 33.8 | render/thread/ms 49.24, render/busy/ms 47.58, encode//sum 41.15, main/busy/ms 33.78, stage/render/cleanup 25.56, render/graph/ms 23.15, stage/main/post_update 22.32, stage/render/graph_submit 18.67, encode/submit 11.54, stage/render/prepare_assets 11.06, stage/main/update 7.76, encode/finish_wall 7.28 |
| measure | 58.05 | 32.6 | render/thread/ms 48.17, render/busy/ms 25.44, stage/render/prepare_views 21.86, render/acquire/ms 21.71, encode//sum 20.33, main/busy/ms 16.40, render/graph/ms 11.77, stage/render/graph_submit 9.59, stage/main/post_update 8.96, stage/render/render 7.38, stage/render/cleanup 6.05, encode/finish_wall 5.44 |
| measure | 59.05 | 35.7 | render/thread/ms 42.65, render/busy/ms 41.06, encode//sum 26.66, main/busy/ms 23.56, render/graph/ms 17.03, stage/render/graph_submit 13.86, stage/main/post_update 12.12, stage/render/prepare_assets 11.66, stage/render/cleanup 11.63, encode/finish_wall 7.83, stage/render/render 7.54, encode/submit 6.15 |
| measure | 59.09 | 32.2 | render/thread/ms 39.90, render/busy/ms 38.82, encode//sum 21.27, stage/render/prepare_assets 19.44, main/busy/ms 19.28, stage/render/render 13.79, render/graph/ms 12.36, stage/render/graph_submit 10.27, stage/main/post_update 10.21, encode/finish_wall 6.30, stage/main/update 5.09, encode/submit 4.01 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ultra2\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_QUALITY=ultra
- before the run: GPU 0%, CPU 7%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **83%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.9 W · 73 °C (max 79) · VRAM 7718 MiB · throttled 100% of samples
