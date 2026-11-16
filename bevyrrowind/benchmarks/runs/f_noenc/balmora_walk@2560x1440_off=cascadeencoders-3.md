# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadeencoders · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5695 frames over 65.7 s · **median 11.28 ms (89 fps)** · mean 11.53 · p95 14.29 · p99 16.45 (1% low 61 fps) · worst 30.57 ms · 43 spikes over 1.5× median · 28.3 stutters/min (work 27.4/min)
- warm-up: 326 frames over 9.3 s, worst 2654.5 ms
- hitches (> 28.2 ms): **4** measured, 41 in warm-up
- timed GPU passes: 7.02 ms of a 11.53 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.28 ms, limit 11 ms
- **FAIL** frame_ms: 31 stutters (> 1.5x moving median of 60) in 1.09 min = 28.33/min, limit 1/min
- pass worst frame 30.57 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.989 | 2.085 | 2.266 | 0.95 | 0.174 |
| main_opaque_pass_3d | 1.569 | 1.645 | 2.051 | 0.95 | 0.057 |
| early prepass | 0.543 | 0.569 | 1.132 | 0.95 | 0.080 |
| atmosphere_luts | 0.308 | 0.323 | 0.435 | 0.95 | 0.004 |
| bloom | 0.308 | 0.322 | 0.327 | 0.95 | 0.064 |
| water_velocity | 0.273 | 0.286 | 0.353 | 0.95 | 0.002 |
| water_sspr | 0.198 | 0.207 | 0.206 | 0.95 | 0.002 |
| ssao | 0.186 | 0.195 | 0.197 | 0.95 | 0.007 |
| auxiliary_views | 0.163 | 0.171 | 0.638 | 0.95 | 0.148 |
| atmos_froxel_inject | 0.141 | 0.148 | 0.148 | 0.95 | 0.001 |
| uniform_allocation | 0.139 | 0.146 | 0.401 | 0.95 | 0.024 |
| clouds_trace | 0.136 | 0.142 | 0.237 | 0.95 | 0.001 |
| water_fft | 0.128 | 0.134 | 0.139 | 0.95 | 0.003 |
| upscaling | 0.117 | 0.122 | 0.130 | 0.95 | 0.004 |
| clouds_composite | 0.103 | 0.108 | 0.110 | 0.95 | 0.002 |
| bin_unpacking | 0.095 | 0.099 | 0.122 | 0.95 | 0.018 |
| tonemapping | 0.083 | 0.087 | 0.090 | 0.95 | 0.003 |
| main_transmissive_pass_3d | 0.080 | 0.084 | 0.160 | 0.95 | 0.008 |
| clustering | 0.063 | 0.066 | 0.069 | 0.95 | 0.019 |
| render_sky | 0.050 | 0.052 | 0.053 | 0.95 | 0.002 |
| clouds_resolve | 0.047 | 0.049 | 0.050 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.428 | 0.422 | 0.11 | 0.001 |
| surface_trail | 0.042 | 0.059 | 0.058 | 0.71 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.95 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.030 | 0.043 | 0.95 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.95 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.228 | 0.234 | 0.11 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.95 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.025 | 0.95 | 0.111 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.95 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.95 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.95 | 0.004 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.95 | 0.002 |
| lightprobe_downsampling_second_pass | 0.002 | 0.016 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_first_pass | 0.001 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.11 | 0.004 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.348 | 16.309 | 52.78 | 58931.1 |
| render/thread/ms | 10.217 | 12.812 | 24.68 | 58183.2 |
| render/busy/ms | 9.626 | 12.230 | 24.13 | 54818.0 |
| main/busy/ms | 9.225 | 11.540 | 28.49 | 52535.1 |
| stage/render/render | 6.508 | 8.369 | 16.53 | 37063.1 |
| stage/main/post_update | 5.782 | 7.735 | 12.83 | 32926.3 |
| render/graph/ms | 5.729 | 7.557 | 15.38 | 32626.2 |
| stage/render/graph_submit | 4.578 | 6.087 | 11.51 | 26073.3 |
| encode/finish_wall | 2.346 | 3.318 | 6.26 | 13360.5 |
| encode/submit | 2.305 | 3.228 | 6.27 | 13125.5 |
| stage/main/update | 2.194 | 2.678 | 22.24 | 12492.6 |
| stage/render/camera_driver | 1.096 | 1.502 | 4.03 | 6243.0 |
| stage/render/prepare_write_phase_buffers | 0.756 | 1.040 | 2.57 | 4303.3 |
| bigspace/total/ms | 0.621 | 1.995 | 4.02 | 3538.5 |
| stage/render/prepare_assets | 0.555 | 0.729 | 11.36 | 3160.1 |
| render/present/ms | 0.498 | 0.650 | 4.03 | 2838.9 |
| stage/render/prepare_resources | 0.447 | 0.554 | 1.48 | 2544.7 |
| stage/render/prepare_meshes | 0.402 | 0.535 | 1.91 | 2288.0 |
| stage/main/first | 0.395 | 0.518 | 5.06 | 2250.6 |
| stage/main/pre_update | 0.331 | 0.496 | 1.11 | 1886.9 |
| terrain/update/ms | 0.300 | 0.393 | 2.37 | 1711.1 |
| stage/render/specialize | 0.279 | 0.417 | 2.51 | 1589.0 |
| controller/feed/ms | 0.273 | 0.362 | 0.98 | 1557.5 |
| terrain/select/ms | 0.267 | 0.349 | 0.80 | 1521.0 |
| bigspace/low_precision/ms | 0.264 | 0.887 | 3.49 | 1504.3 |
| stage/render/prepare_bind_groups | 0.258 | 0.429 | 2.50 | 1470.8 |
| stream/drive/ms | 0.254 | 0.344 | 20.19 | 1446.5 |
| stage/main/fixed_main_loop | 0.226 | 0.382 | 1.78 | 1285.9 |
| anim/evaluate/ms | 0.213 | 0.370 | 3.86 | 1215.7 |
| stage/render/queue | 0.209 | 0.366 | 2.50 | 1189.1 |
| bigspace/high_precision/ms | 0.202 | 0.884 | 3.60 | 1151.3 |
| stream/tick/ms | 0.196 | 0.265 | 0.61 | 1117.2 |
| far/select/ms | 0.192 | 0.264 | 0.64 | 1095.3 |
| stage/render/create_views | 0.186 | 0.245 | 0.94 | 1057.6 |
| stage/render/extract_commands | 0.176 | 0.216 | 0.56 | 1002.5 |
| stage/render/prepare_flush | 0.175 | 0.351 | 1.01 | 995.4 |
| stage/render/prepare_batch_phases | 0.165 | 0.239 | 0.58 | 941.9 |
| anim/evaluate_cpu/ms | 0.147 | 0.190 | 0.48 | 839.1 |
| controller/move/ms | 0.140 | 0.287 | 0.56 | 795.4 |
| stage/render/prepare_views | 0.134 | 0.174 | 0.38 | 761.8 |
| stage/main/last | 0.121 | 0.196 | 1.67 | 689.2 |
| render/acquire/ms | 0.092 | 0.122 | 0.35 | 526.3 |
| stage/render/phase_sort | 0.077 | 0.106 | 0.40 | 438.4 |
| pipelines/compile_ms | 0.052 | 0.000 | 123.65 | 294.3 |
| physics/step/ms | 0.050 | 0.088 | 0.64 | 285.9 |
| pipelines/front_ms | 0.050 | 0.000 | 119.83 | 282.8 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.076 | 0.36 | 271.6 |
| stage/render/cleanup | 0.032 | 0.097 | 0.96 | 185.0 |
| physics/insert/ms | 0.032 | 0.366 | 0.88 | 184.5 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.41 | 179.6 |
| weather/map/ms | 0.026 | 0.034 | 0.25 | 148.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.26 | 85.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.05 | 65.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.57 | 63.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 60.4 |
| controller/avoid/ms | 0.010 | 0.014 | 0.06 | 58.9 |
| script/update/ms | 0.007 | 0.010 | 0.04 | 38.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.08 | 28.7 |
| script/ui/ms | 0.005 | 0.007 | 0.06 | 27.9 |
| script/input/ms | 0.005 | 0.006 | 0.13 | 27.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 26.1 |
| controller/write/ms | 0.004 | 0.005 | 0.06 | 20.7 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 13.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.82 | 11.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.20 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.3 |
| stage/render/prepare | 0.001 | 0.002 | 0.10 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.11 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 21 |
| controller/movers/count | count | 20 | 22 | 21 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 145 | 155 | 149 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5661 |
| crowd/c1/meshes/count | count | 1185 | 1281 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4631 | 10.3 k | 6528 |
| crowd/joints_uploaded/count | count | 4615 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.88 | 5.98 | 1.70 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.26 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.57 | 0.01 |
| encode/finish_critical | ms | 1.88 | 5.98 | 1.70 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.41 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.74 | 28.49 | 10.85 |
| gpu/timed/ms | ms | 8.01 | 11.09 | 8.52 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 819 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 819 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3419 | 5115 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3419 | 5115 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 201 | 4 |
| phases/shadow/bins | count | 4401 | 5423 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.1 k | 12.6 k | 5442 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 194 | 233 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2350 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 559 | 29 |
| preprocess/bind_groups_created | count | 13 | 559 | 29 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 225 | 100 |
| process/cpu_usage | % | 25.96 | 38.94 | 23.70 |
| process/mem_usage | GiB | 5.35 | 5.52 | 5.52 |
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
| started_load_count |  | 4915 | 5008 | 5008 |
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
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.81 | 63.30 | 31.32 |
| system/mem_usage | % | 68.30 | 69.22 | 69.01 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 67 | 248 | 54 |
| views/all/bins | count | 6987 | 8367 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 25.1 k | 13.1 k |
| views/all/pipelines | count | 239 | 280 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 22.2 k | 8165 |
| views/cascade/batch_sets | count | 10 | 33 | 4 |
| views/cascade/bins | count | 2219 | 2880 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7647 | 9640 | 5442 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 521 | 9640 | 239 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2185 | 2802 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2428 | 3099 | 0 |
| views/lamp_face/pipelines | count | 170 | 204 | 0 |
| views/lamp_face/work_items | count | 0 | 2887 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2860 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8462 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8622 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 58.05 | 230 | 330.6 k |
| anim/events/count | count | 0.10 | 3 | 550 |
| assets/Image/added | count | 0.10 | 36 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.71 | 152 | 4061 |
| assets/Mesh/removed | count | 0.85 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.81 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.82 | 1415 | 4675 |
| encode//count | count | 39.52 | 272 | 225.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5695 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5695 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5695 |
| physics/inserts | count | 0.39 | 107 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 161 | 2335 |
| physics/steps | count | 0.69 | 2 | 3939 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 5944 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1303 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.2 ms; slowest 45, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.65 | 2654.5 | main/busy/ms 2624, stage/main/post_update 2522, stage/main/update 95.21, surface/advance_cpu_ms 66.46, stage/main/pre_update 2.72, stage/main/last 1.11, surface/cell_upload_ms 0.76, stage/main/fixed_main_loop 0.68, stage/main/first 0.54, stream/drive/ms 0.50, weather/map/ms 0.12, stream/tick/ms 0.11 |
| warm-up | 3.42 | 766.8 | main/busy/ms 2767, stage/main/post_update 2546, render/thread/ms 736, render/busy/ms 566, stage/render/prepare_assets 530, stage/main/update 207, stage/render/prepare_views 165, render/acquire/ms 164, surface/advance_cpu_ms 66.46, stage/render/prepare_resources 21.50, stage/render/render 13.21, encode//sum 7.90 |
| warm-up | 3.51 | 89.9 | render/thread/ms 806, render/busy/ms 629, stage/render/prepare_assets 577, main/busy/ms 170, stage/render/prepare_views 166, render/acquire/ms 165, stage/main/update 118, stage/main/post_update 41.56, stage/render/prepare_resources 26.74, stage/render/render 24.98, terrain/update/ms 18.61, encode//sum 12.51 |
| warm-up | 3.65 | 111.6 | pipelines/wait_ms 1934, pipelines/compile_ms 856, pipelines/front_ms 805, render/thread/ms 127, render/busy/ms 124, main/busy/ms 56.77, pipelines/driver_ms 51.22, stage/render/prepare_resources 37.08, stage/render/render 30.84, stage/main/post_update 30.74, stage/render/prepare_assets 24.27, render/graph/ms 22.58 |
| warm-up | 3.75 | 101.5 | pipelines/wait_ms 6373, pipelines/compile_ms 2140, pipelines/front_ms 2028, render/thread/ms 196, render/busy/ms 194, main/busy/ms 119, pipelines/driver_ms 113, stage/render/prepare_resources 77.79, stage/main/update 75.08, stage/render/render 35.31, stage/render/prepare_assets 35.13, render/graph/ms 32.67 |
| warm-up | 3.87 | 116.7 | pipelines/wait_ms 4785, pipelines/compile_ms 2417, pipelines/front_ms 2291, render/thread/ms 206, render/busy/ms 203, pipelines/driver_ms 125, main/busy/ms 125, stage/render/prepare_assets 109, stage/main/update 79.03, stage/render/prepare_resources 45.13, stage/main/post_update 29.05, stage/render/render 27.06 |
| warm-up | 3.91 | 44.5 | pipelines/compile_ms 1205, pipelines/front_ms 1136, pipelines/wait_ms 518, render/thread/ms 156, render/busy/ms 153, stage/render/prepare_assets 129, pipelines/driver_ms 68.61, main/busy/ms 63.40, stage/main/post_update 31.52, stage/render/render 18.66, stage/main/update 17.32, render/graph/ms 16.08 |
| warm-up | 3.96 | 48.1 | pipelines/compile_ms 1171, pipelines/front_ms 1089, pipelines/wait_ms 244, render/thread/ms 87.87, render/busy/ms 85.43, pipelines/driver_ms 81.88, stage/render/prepare_assets 67.84, main/busy/ms 54.56, stage/main/post_update 29.87, stage/main/update 18.11, encode//sum 15.04, stage/render/render 11.61 |
| warm-up | 4.07 | 28.4 | pipelines/compile_ms 260, pipelines/front_ms 252, render/thread/ms 50.67, render/busy/ms 49.11, main/busy/ms 47.18, stage/main/update 35.29, stream/drive/ms 32.01, stage/render/prepare_assets 16.35, stage/render/render 12.59, render/graph/ms 10.49, stage/render/prepare_bind_groups 7.89, pipelines/driver_ms 7.64 |
| warm-up | 4.13 | 58.2 | pipelines/compile_ms 1294, pipelines/front_ms 1245, render/thread/ms 63.34, render/busy/ms 61.85, main/busy/ms 51.77, pipelines/driver_ms 48.53, stage/main/update 37.63, stream/drive/ms 32.31, stage/render/prepare_assets 28.48, stage/render/render 13.89, render/graph/ms 11.98, pipelines/wait_ms 11.83 |
| warm-up | 4.16 | 30.8 | pipelines/compile_ms 1340, pipelines/front_ms 1285, render/thread/ms 62.39, render/busy/ms 60.90, main/busy/ms 58.80, pipelines/driver_ms 54.60, stage/render/prepare_assets 42.70, stage/main/update 41.24, stream/drive/ms 35.14, pipelines/wait_ms 14.92, stage/render/render 13.61, render/graph/ms 11.57 |
| warm-up | 4.23 | 71.5 | pipelines/compile_ms 760, pipelines/front_ms 733, render/thread/ms 68.34, render/busy/ms 66.47, main/busy/ms 59.48, stage/render/prepare_assets 45.74, stage/main/update 41.06, stream/drive/ms 36.19, pipelines/driver_ms 27.02, stage/render/render 14.09, stage/main/post_update 11.54, render/graph/ms 11.46 |
| warm-up | 4.35 | 96.2 | pipelines/compile_ms 790, pipelines/front_ms 763, render/thread/ms 79.40, render/busy/ms 76.05, main/busy/ms 65.20, stage/render/prepare_assets 52.32, stage/main/update 38.88, pipelines/driver_ms 26.90, stage/main/post_update 18.13, stage/render/render 15.33, stream/drive/ms 14.27, render/graph/ms 12.45 |
| warm-up | 4.40 | 56.9 | pipelines/compile_ms 1417, pipelines/front_ms 1377, render/thread/ms 98.05, render/busy/ms 96.19, stage/render/prepare_assets 78.79, main/busy/ms 65.65, stage/main/update 42.12, pipelines/driver_ms 39.44, stage/main/post_update 15.82, stage/render/render 11.53, render/graph/ms 9.14, stage/render/graph_submit 6.64 |
| warm-up | 4.46 | 61.6 | pipelines/compile_ms 1179, pipelines/front_ms 1148, render/thread/ms 80.49, render/busy/ms 78.74, stage/render/prepare_assets 63.22, main/busy/ms 52.73, stage/main/update 35.82, pipelines/driver_ms 31.44, stage/render/render 9.15, stage/main/post_update 8.82, render/graph/ms 7.04, encode//sum 5.77 |
| warm-up | 4.56 | 95.4 | pipelines/compile_ms 724, pipelines/front_ms 701, render/thread/ms 133, render/busy/ms 132, main/busy/ms 72.57, stage/main/update 40.23, stage/render/prepare_resources 38.49, stage/render/prepare_assets 38.03, pipelines/driver_ms 22.49, stage/render/prepare_flush 21.06, stage/render/render 11.52, stage/render/prepare_bind_groups 11.48 |
| warm-up | 4.61 | 47.4 | pipelines/compile_ms 1213, pipelines/front_ms 1173, render/thread/ms 128, render/busy/ms 126, main/busy/ms 80.77, pipelines/driver_ms 40.88, stage/render/prepare_resources 39.23, stage/main/update 35.57, stage/render/prepare_assets 23.75, stage/render/prepare_flush 21.37, stage/render/prepare_bind_groups 15.23, stage/main/pre_update 13.54 |
| warm-up | 4.66 | 54.9 | pipelines/compile_ms 1520, pipelines/front_ms 1470, render/thread/ms 77.61, render/busy/ms 75.96, main/busy/ms 60.93, pipelines/driver_ms 50.35, stage/render/prepare_assets 37.70, pipelines/wait_ms 31.29, stage/main/update 18.17, stage/main/post_update 17.57, stage/main/pre_update 16.71, encode//sum 12.60 |
| warm-up | 4.73 | 68.3 | pipelines/compile_ms 2184, pipelines/front_ms 2121, pipelines/wait_ms 363, render/thread/ms 106, render/busy/ms 104, main/busy/ms 71.18, pipelines/driver_ms 63.27, stage/main/post_update 41.80, stage/render/prepare_assets 35.32, encode//sum 22.00, stage/main/update 18.52, stage/render/render 18.19 |
| warm-up | 4.83 | 101.2 | pipelines/compile_ms 2757, pipelines/front_ms 2677, pipelines/wait_ms 1309, main/busy/ms 167, render/thread/ms 162, render/busy/ms 158, stage/main/update 125, encode//sum 86.77, stream/drive/ms 83.97, pipelines/driver_ms 80.02, stage/render/render 47.65, render/graph/ms 44.00 |
| warm-up | 4.96 | 123.9 | pipelines/compile_ms 2799, pipelines/front_ms 2705, pipelines/wait_ms 1438, render/thread/ms 168, render/busy/ms 165, main/busy/ms 148, stage/main/update 127, pipelines/driver_ms 93.76, encode//sum 88.76, stream/drive/ms 84.59, stage/render/prepare_assets 58.24, stage/render/render 50.97 |
| warm-up | 4.98 | 28.2 | pipelines/compile_ms 1775, pipelines/front_ms 1711, pipelines/wait_ms 465, render/thread/ms 96.58, render/busy/ms 94.75, main/busy/ms 81.41, pipelines/driver_ms 64.42, stage/main/update 60.90, stream/drive/ms 49.69, stage/render/prepare_assets 43.39, stage/render/render 23.61, encode//sum 22.50 |
| warm-up | 5.08 | 100.9 | pipelines/compile_ms 773, pipelines/front_ms 747, main/busy/ms 89.84, render/thread/ms 67.62, stage/main/update 67.53, render/busy/ms 64.15, stream/drive/ms 49.55, stage/render/prepare_assets 37.86, pipelines/wait_ms 37.60, pipelines/driver_ms 26.37, stage/render/render 16.44, stage/main/post_update 14.03 |
| warm-up | 5.15 | 61.7 | pipelines/compile_ms 1113, pipelines/front_ms 1055, render/thread/ms 96.11, main/busy/ms 94.56, stage/main/update 70.84, render/busy/ms 67.66, pipelines/driver_ms 58.04, stream/drive/ms 48.15, stage/render/prepare_assets 46.13, stage/render/render 38.44, pipelines/wait_ms 37.96, render/present/ms 28.20 |
| warm-up | 5.21 | 60.1 | pipelines/compile_ms 878, pipelines/front_ms 783, pipelines/driver_ms 95.43, render/thread/ms 89.50, main/busy/ms 82.45, stage/main/update 64.86, render/busy/ms 62.87, stream/drive/ms 47.69, stage/render/prepare_assets 38.86, stage/render/render 38.32, render/present/ms 26.38, encode//sum 18.04 |
| warm-up | 5.24 | 35.8 | pipelines/compile_ms 697, pipelines/front_ms 630, main/busy/ms 76.27, pipelines/driver_ms 67.72, render/thread/ms 67.18, render/busy/ms 65.30, stage/main/update 52.92, stage/render/prepare_assets 35.78, stream/drive/ms 34.71, stage/main/post_update 16.76, encode//sum 16.19, stage/render/render 15.16 |
| warm-up | 5.32 | 75.7 | pipelines/compile_ms 782, pipelines/front_ms 757, main/busy/ms 82.66, render/thread/ms 61.95, render/busy/ms 59.96, stage/main/update 49.96, stream/drive/ms 35.31, stage/render/prepare_assets 30.64, pipelines/driver_ms 25.04, stage/main/post_update 18.55, encode//sum 16.79, stage/render/render 13.80 |
| warm-up | 5.35 | 36.1 | pipelines/compile_ms 545, pipelines/front_ms 527, render/thread/ms 64.45, render/busy/ms 62.71, main/busy/ms 52.65, stage/render/prepare_assets 24.95, stage/main/post_update 19.19, encode//sum 17.74, pipelines/driver_ms 17.43, stage/render/render 16.97, render/graph/ms 14.39, stage/main/update 13.74 |
| warm-up | 5.40 | 29.5 | pipelines/compile_ms 584, pipelines/front_ms 568, render/thread/ms 41.48, render/busy/ms 39.99, main/busy/ms 31.61, pipelines/driver_ms 15.86, encode//sum 15.56, stage/main/update 14.62, stage/render/prepare_assets 13.82, stage/render/render 13.40, render/graph/ms 11.40, stage/main/post_update 10.67 |
| warm-up | 5.50 | 50.4 | pipelines/compile_ms 202, pipelines/front_ms 193, render/thread/ms 49.29, render/busy/ms 47.42, main/busy/ms 38.84, stage/main/update 21.96, stage/render/prepare_assets 21.68, stage/render/render 16.51, encode//sum 15.21, render/graph/ms 14.04, stage/main/post_update 11.80, stage/render/graph_submit 11.58 |
| warm-up | 5.53 | 37.5 | pipelines/compile_ms 253, pipelines/front_ms 242, render/thread/ms 61.97, render/busy/ms 60.31, main/busy/ms 40.95, stage/render/prepare_assets 33.05, stage/main/update 22.31, stage/render/render 16.57, encode//sum 15.59, render/graph/ms 14.42, stage/main/post_update 13.40, stream/drive/ms 12.94 |
| warm-up | 5.57 | 36.7 | pipelines/compile_ms 515, pipelines/front_ms 498, render/thread/ms 62.01, render/busy/ms 60.43, main/busy/ms 49.10, stage/render/prepare_assets 32.90, stage/main/update 23.87, pipelines/driver_ms 17.00, encode//sum 16.74, stage/render/render 14.55, stage/main/post_update 14.52, stream/drive/ms 12.29 |
| warm-up | 5.62 | 45.3 | pipelines/compile_ms 696, pipelines/front_ms 674, render/thread/ms 71.54, render/busy/ms 68.28, main/busy/ms 47.11, stage/render/prepare_assets 37.18, pipelines/driver_ms 21.86, stage/main/post_update 19.44, stage/render/render 17.26, stage/main/update 16.16, encode//sum 15.89, render/graph/ms 13.04 |
| warm-up | 5.65 | 30.8 | pipelines/compile_ms 488, pipelines/front_ms 474, render/thread/ms 68.47, main/busy/ms 63.70, render/busy/ms 63.35, pipelines/wait_ms 24.33, stage/render/prepare_assets 24.18, stage/render/render 22.24, stage/main/post_update 19.17, stage/main/update 18.87, encode//sum 17.84, render/graph/ms 15.92 |
| warm-up | 5.69 | 43.1 | pipelines/compile_ms 858, pipelines/front_ms 828, main/busy/ms 58.57, render/thread/ms 57.83, render/busy/ms 54.58, pipelines/wait_ms 30.20, pipelines/driver_ms 30.03, stage/render/prepare_assets 25.02, stage/main/update 17.57, stage/render/render 17.17, encode//sum 17.14, stage/main/post_update 16.75 |
| warm-up | 5.74 | 35.3 | pipelines/compile_ms 853, pipelines/front_ms 832, render/thread/ms 44.71, render/busy/ms 42.99, main/busy/ms 32.30, pipelines/driver_ms 20.92, stage/render/prepare_assets 18.74, encode//sum 17.73, stage/main/post_update 15.63, stage/render/render 15.33, render/graph/ms 12.96, stage/main/update 10.55 |
| warm-up | 6.33 | 30.7 | render/thread/ms 34.22, render/busy/ms 33.02, main/busy/ms 27.05, stage/render/prepare_assets 15.79, encode//sum 13.61, stage/main/update 11.76, stage/main/post_update 9.95, stage/render/render 9.07, render/graph/ms 7.57, stage/render/graph_submit 5.59, stream/drive/ms 3.67, encode/submit 2.99 |
| warm-up | 6.85 | 30.0 | render/thread/ms 41.41, render/busy/ms 40.09, main/busy/ms 37.48, stage/main/update 19.73, stage/render/prepare_assets 19.32, encode//sum 18.54, stage/main/post_update 13.48, stream/drive/ms 13.40, stage/render/render 12.63, render/graph/ms 10.78, stage/render/graph_submit 8.19, encode/finish_wall 4.25 |
| warm-up | 6.88 | 31.5 | render/thread/ms 56.64, render/busy/ms 55.32, main/busy/ms 39.45, stage/render/prepare_assets 34.62, stage/main/update 21.79, encode//sum 19.04, stream/drive/ms 13.81, stage/main/post_update 12.95, stage/render/render 12.54, render/graph/ms 10.61, stage/render/graph_submit 8.31, encode/finish_wall 4.37 |
| warm-up | 6.92 | 29.2 | render/thread/ms 36.20, render/busy/ms 34.77, main/busy/ms 28.58, encode//sum 23.53, stage/render/prepare_assets 14.29, stage/main/update 14.22, stage/render/render 13.64, render/graph/ms 11.68, stage/render/graph_submit 9.46, stage/main/post_update 9.34, stream/drive/ms 7.61, encode/submit 5.05 |
| warm-up | 7.10 | 34.0 | main/busy/ms 40.99, render/thread/ms 26.58, render/busy/ms 25.39, stage/main/update 22.58, encode//sum 21.16, stream/drive/ms 13.94, stage/main/post_update 13.35, stage/render/render 13.19, stream/spawn/ms 12.72, render/graph/ms 11.60, stage/render/graph_submit 9.06, stage/render/prepare_assets 5.15 |
| measure | 18.18 | 29.3 | main/busy/ms 38.46, stage/main/update 24.29, encode//sum 23.59, render/thread/ms 21.52, stream/drive/ms 20.42, render/busy/ms 20.31, stage/render/render 13.11, render/graph/ms 11.46, stage/main/post_update 11.42, stage/render/graph_submit 8.89, encode/submit 4.53, encode/finish_wall 4.45 |
| measure | 24.04 | 28.5 | encode//sum 38.76, render/thread/ms 36.33, render/busy/ms 35.09, main/busy/ms 30.38, stage/render/render 17.57, render/graph/ms 15.80, stage/main/post_update 13.38, stage/render/graph_submit 12.63, stage/main/update 12.32, encode/finish_wall 7.21, stage/render/prepare_assets 6.15, encode/submit 5.50 |
| measure | 59.01 | 28.4 | render/thread/ms 40.15, render/busy/ms 38.22, main/busy/ms 38.19, encode//sum 29.23, stage/main/post_update 24.17, stage/render/render 21.73, render/graph/ms 19.27, stage/render/graph_submit 14.53, stage/main/update 8.78, encode/finish_wall 8.27, encode/submit 6.52, stage/render/prepare_assets 5.23 |
| measure | 59.04 | 30.6 | render/thread/ms 42.74, render/busy/ms 41.47, main/busy/ms 31.29, encode//sum 21.68, stage/main/post_update 18.27, stage/render/prepare_assets 15.98, stage/render/render 14.78, render/graph/ms 13.18, stage/render/graph_submit 10.02, stage/main/update 7.19, encode/finish_wall 5.99, encode/submit 4.13 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_noenc\balmora_walk@2560x1440_off=cascadeencoders-3 --bench-baseline none --off cascadeencoders
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 58%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.5 W · 72 °C (max 78) · VRAM 6002 MiB · throttled 100% of samples
