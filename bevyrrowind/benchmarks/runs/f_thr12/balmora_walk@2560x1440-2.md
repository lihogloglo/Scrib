# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5656 frames over 65.6 s · **median 11.35 ms (88 fps)** · mean 11.61 · p95 13.88 · p99 15.84 (1% low 63 fps) · worst 32.74 ms · 28 spikes over 1.5× median · 21.9 stutters/min (work 21.0/min)
- warm-up: 322 frames over 8.9 s, worst 2521.9 ms
- hitches (> 28.4 ms): **3** measured, 39 in warm-up
- timed GPU passes: 7.37 ms of a 11.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.35 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- pass worst frame 32.74 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.055 | 2.081 | 2.232 | 0.99 | 0.174 |
| main_opaque_pass_3d | 1.632 | 1.653 | 2.049 | 0.99 | 0.056 |
| early prepass | 0.537 | 0.544 | 0.778 | 0.99 | 0.080 |
| bloom | 0.316 | 0.321 | 0.326 | 0.99 | 0.063 |
| atmosphere_luts | 0.312 | 0.316 | 0.374 | 0.99 | 0.003 |
| water_velocity | 0.281 | 0.285 | 0.353 | 0.99 | 0.002 |
| water_sspr | 0.204 | 0.206 | 0.206 | 0.99 | 0.002 |
| ssao | 0.190 | 0.192 | 0.195 | 0.99 | 0.007 |
| auxiliary_views | 0.151 | 0.153 | 0.586 | 0.99 | 0.136 |
| uniform_allocation | 0.150 | 0.152 | 0.434 | 0.99 | 0.024 |
| per_view_shadow_pass_cascade1 | 0.146 | 0.580 | 0.657 | 0.25 | 0.042 |
| atmos_froxel_inject | 0.145 | 0.147 | 0.147 | 0.99 | 0.001 |
| clouds_trace | 0.141 | 0.142 | 0.233 | 0.99 | 0.001 |
| water_fft | 0.134 | 0.136 | 0.141 | 0.99 | 0.003 |
| upscaling | 0.116 | 0.118 | 0.127 | 0.99 | 0.004 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 0.99 | 0.002 |
| bin_unpacking | 0.100 | 0.101 | 0.122 | 0.99 | 0.017 |
| tonemapping | 0.086 | 0.087 | 0.090 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.083 | 0.084 | 0.161 | 0.99 | 0.008 |
| clustering | 0.065 | 0.065 | 0.068 | 0.99 | 0.018 |
| render_sky | 0.052 | 0.053 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.425 | 0.422 | 0.12 | 0.001 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.043 | 0.058 | 0.057 | 0.74 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.99 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.031 | 0.043 | 0.99 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.228 | 0.236 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.025 | 0.99 | 0.113 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.004 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.012 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.038 | 0.050 | 0.01 | 0.013 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.044 | 0.047 | 0.00 | 0.013 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 12.652 | 19.660 | 122.66 | 71561.2 |
| render/thread/ms | 10.341 | 12.511 | 31.61 | 58489.9 |
| render/busy/ms | 9.781 | 11.934 | 30.81 | 55321.0 |
| main/busy/ms | 8.933 | 11.052 | 28.56 | 50524.7 |
| stage/render/render | 6.660 | 8.121 | 22.68 | 37667.9 |
| render/graph/ms | 5.912 | 7.299 | 21.52 | 33437.5 |
| stage/main/post_update | 5.500 | 7.361 | 10.55 | 31109.4 |
| stage/render/graph_submit | 4.766 | 5.862 | 16.95 | 26957.7 |
| encode/finish_wall | 2.504 | 3.161 | 10.59 | 14160.2 |
| encode/submit | 2.310 | 3.128 | 6.39 | 13067.5 |
| stage/main/update | 2.183 | 2.637 | 22.43 | 12344.6 |
| stage/render/camera_driver | 1.090 | 1.487 | 4.50 | 6166.5 |
| stage/render/prepare_write_phase_buffers | 0.764 | 1.034 | 2.53 | 4318.9 |
| bigspace/total/ms | 0.602 | 1.712 | 3.72 | 3407.4 |
| stage/render/prepare_assets | 0.551 | 0.707 | 10.80 | 3119.1 |
| render/present/ms | 0.468 | 0.563 | 2.32 | 2644.5 |
| stage/render/prepare_resources | 0.451 | 0.542 | 1.37 | 2552.3 |
| stage/main/first | 0.404 | 0.521 | 5.48 | 2284.6 |
| stage/render/prepare_meshes | 0.385 | 0.489 | 1.90 | 2176.4 |
| stage/main/pre_update | 0.328 | 0.484 | 1.46 | 1857.5 |
| terrain/update/ms | 0.315 | 0.423 | 1.90 | 1781.7 |
| terrain/select/ms | 0.281 | 0.381 | 0.79 | 1588.8 |
| stage/render/specialize | 0.274 | 0.401 | 2.46 | 1552.0 |
| controller/feed/ms | 0.267 | 0.337 | 0.65 | 1509.3 |
| stage/render/prepare_bind_groups | 0.258 | 0.427 | 2.63 | 1456.6 |
| stream/drive/ms | 0.255 | 0.344 | 20.45 | 1440.1 |
| bigspace/high_precision/ms | 0.245 | 0.782 | 3.39 | 1388.0 |
| stage/main/fixed_main_loop | 0.222 | 0.359 | 1.88 | 1255.2 |
| stage/render/queue | 0.205 | 0.349 | 2.71 | 1158.0 |
| bigspace/low_precision/ms | 0.202 | 0.564 | 3.00 | 1143.4 |
| far/select/ms | 0.199 | 0.266 | 0.58 | 1126.1 |
| stream/tick/ms | 0.197 | 0.267 | 0.62 | 1112.7 |
| stage/render/create_views | 0.181 | 0.238 | 0.81 | 1025.8 |
| stage/render/prepare_flush | 0.180 | 0.349 | 0.80 | 1015.3 |
| anim/evaluate/ms | 0.172 | 0.286 | 3.23 | 974.6 |
| stage/render/extract_commands | 0.171 | 0.205 | 0.51 | 967.0 |
| stage/render/prepare_batch_phases | 0.161 | 0.220 | 0.43 | 912.9 |
| anim/evaluate_cpu/ms | 0.153 | 0.196 | 0.42 | 866.2 |
| stage/render/prepare_views | 0.135 | 0.172 | 0.44 | 761.0 |
| controller/move/ms | 0.128 | 0.281 | 0.72 | 726.2 |
| stage/main/last | 0.121 | 0.190 | 2.71 | 684.9 |
| render/acquire/ms | 0.093 | 0.123 | 0.34 | 524.4 |
| stage/render/phase_sort | 0.077 | 0.103 | 0.33 | 434.0 |
| pipelines/compile_ms | 0.051 | 0.000 | 119.11 | 287.7 |
| physics/step/ms | 0.049 | 0.082 | 0.50 | 275.5 |
| pipelines/front_ms | 0.049 | 0.000 | 114.44 | 275.4 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.41 | 266.2 |
| encode/resolve_diagnostics/sum | 0.035 | 0.044 | 9.50 | 198.7 |
| physics/insert/ms | 0.035 | 0.380 | 0.93 | 195.3 |
| stage/render/cleanup | 0.033 | 0.108 | 0.78 | 184.6 |
| weather/map/ms | 0.027 | 0.035 | 0.27 | 155.0 |
| encode/camera_driver_span_begin/sum | 0.017 | 0.023 | 0.58 | 93.8 |
| encode/camera_driver_span_end/sum | 0.013 | 0.020 | 0.64 | 71.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.19 | 62.3 |
| anim/apply/ms | 0.011 | 0.022 | 0.20 | 61.6 |
| controller/avoid/ms | 0.011 | 0.014 | 0.05 | 60.8 |
| script/update/ms | 0.007 | 0.009 | 0.25 | 37.6 |
| stream/despawn/ms | 0.005 | 0.000 | 0.85 | 28.8 |
| script/ui/ms | 0.005 | 0.006 | 0.15 | 27.3 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 26.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 26.7 |
| controller/write/ms | 0.003 | 0.005 | 0.13 | 19.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 15.4 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.66 | 12.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.07 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 21 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 145 | 158 | 148 |
| crowd/c1/joints/count | count | 5531 | 6045 | 5613 |
| crowd/c1/meshes/count | count | 1166 | 1315 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4601 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.21 | 10.22 | 2.36 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.58 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.64 | 0.01 |
| encode/finish_critical | ms | 2.21 | 10.22 | 2.36 |
| encode/resolve_diagnostics/max | ms | 0.03 | 9.50 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.74 | 30.81 | 9.04 |
| gpu/timed/ms | ms | 8.64 | 12.61 | 8.89 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 822 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 822 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1282 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3420 | 5132 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1282 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3420 | 5132 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 211 | 4 |
| phases/shadow/bins | count | 4388 | 5524 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.1 k | 12.6 k | 5438 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 189 | 237 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 580 | 6 |
| preprocess/bind_groups_created | count | 14 | 580 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 132 | 203 | 123 |
| process/cpu_usage | % | 28.37 | 48.44 | 22.60 |
| process/mem_usage | GiB | 5.33 | 5.48 | 5.48 |
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
| started_load_count |  | 4915 | 5009 | 5009 |
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
| system/cpu_usage | % | 35.12 | 54.13 | 32.35 |
| system/mem_usage | % | 50.40 | 50.96 | 50.88 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 66 | 258 | 54 |
| views/all/bins | count | 7049 | 8434 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.3 k | 24.9 k | 13.1 k |
| views/all/pipelines | count | 233 | 284 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 22.4 k | 8159 |
| views/cascade/batch_sets | count | 10 | 33 | 4 |
| views/cascade/bins | count | 2219 | 2883 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7656 | 9651 | 5438 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 526 | 9649 | 239 |
| views/lamp_face/batch_sets | count | 0 | 186 | 0 |
| views/lamp_face/bins | count | 2163 | 2845 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2424 | 3157 | 0 |
| views/lamp_face/pipelines | count | 164 | 209 | 0 |
| views/lamp_face/work_items | count | 0 | 2852 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2862 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8484 | 13.0 k | 7612 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8642 | 13.3 k | 7920 |
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
| anim/bones_written/count | count | 52.95 | 337 | 299.5 k |
| anim/events/count | count | 0.09 | 3 | 530 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.83 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1389 | 4675 |
| encode//count | count | 39.47 | 273 | 223.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5656 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5656 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5656 |
| physics/inserts | count | 0.40 | 108 | 2270 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5985 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1312 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.4 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2521.9 | main/busy/ms 2495, stage/main/post_update 2392, stage/main/update 97.07, surface/advance_cpu_ms 64.97, stage/main/pre_update 2.88, stage/main/last 1.16, surface/cell_upload_ms 0.71, stage/main/first 0.51, stream/drive/ms 0.49, stage/main/fixed_main_loop 0.39, weather/map/ms 0.20, bigspace/total/ms 0.13 |
| warm-up | 3.25 | 728.6 | main/busy/ms 2623, stage/main/post_update 2413, render/thread/ms 697, render/busy/ms 535, stage/render/prepare_assets 499, stage/main/update 197, stage/render/prepare_views 159, render/acquire/ms 158, surface/advance_cpu_ms 64.97, stage/render/prepare_resources 21.73, encode//sum 15.37, stage/render/render 12.19 |
| warm-up | 3.33 | 82.7 | render/thread/ms 763, render/busy/ms 598, stage/render/prepare_assets 544, stage/render/prepare_views 160, render/acquire/ms 159, main/busy/ms 149, stage/main/update 106, stage/main/post_update 33.08, stage/render/prepare_resources 27.02, stage/render/render 22.74, encode//sum 20.89, terrain/update/ms 16.43 |
| warm-up | 3.51 | 148.4 | pipelines/wait_ms 3641, pipelines/compile_ms 1297, pipelines/front_ms 1213, render/thread/ms 159, render/busy/ms 149, pipelines/driver_ms 83.73, main/busy/ms 74.05, stage/render/prepare_write_phase_buffers 50.89, stage/render/prepare_resources 38.84, stage/main/post_update 33.29, stage/main/update 28.85, stage/render/prepare_bind_groups 19.36 |
| warm-up | 3.57 | 67.2 | pipelines/wait_ms 5745, pipelines/compile_ms 2186, pipelines/front_ms 2051, render/thread/ms 202, render/busy/ms 200, main/busy/ms 138, pipelines/driver_ms 135, stage/main/update 84.13, stage/render/prepare_resources 64.25, stage/render/prepare_write_phase_buffers 50.91, stage/render/prepare_assets 34.09, stage/main/fixed_main_loop 26.07 |
| warm-up | 3.72 | 149.3 | pipelines/compile_ms 2722, pipelines/front_ms 2520, pipelines/wait_ms 2419, pipelines/driver_ms 202, render/thread/ms 178, render/busy/ms 135, main/busy/ms 114, stage/render/prepare_assets 90.89, stage/main/update 65.85, stage/render/render 53.37, render/present/ms 42.06, stage/render/prepare_resources 28.27 |
| warm-up | 3.83 | 42.8 | pipelines/compile_ms 210, pipelines/front_ms 195, render/thread/ms 50.30, render/busy/ms 35.45, stage/render/render 23.00, main/busy/ms 21.84, stage/render/prepare_assets 17.58, pipelines/driver_ms 14.96, render/present/ms 14.60, stage/main/post_update 12.60, encode//sum 7.77, render/graph/ms 7.60 |
| warm-up | 3.93 | 86.0 | pipelines/compile_ms 1031, pipelines/front_ms 990, render/thread/ms 52.47, render/busy/ms 51.07, main/busy/ms 50.59, pipelines/driver_ms 41.43, stage/render/prepare_assets 39.62, stage/main/update 37.39, stream/drive/ms 33.16, stage/main/post_update 7.24, stage/render/render 6.89, encode//sum 6.15 |
| warm-up | 3.96 | 34.8 | pipelines/compile_ms 812, pipelines/front_ms 784, render/thread/ms 74.46, render/busy/ms 71.99, stage/render/prepare_assets 52.23, main/busy/ms 41.40, stage/main/update 27.51, pipelines/driver_ms 27.38, stream/drive/ms 19.45, stage/render/render 14.10, render/graph/ms 11.40, stage/render/graph_submit 8.07 |
| warm-up | 3.99 | 32.4 | pipelines/compile_ms 518, pipelines/front_ms 497, render/thread/ms 57.34, render/busy/ms 54.93, main/busy/ms 42.30, stage/render/prepare_assets 32.11, stage/main/update 27.40, pipelines/driver_ms 20.52, stream/drive/ms 19.37, stage/render/render 16.57, render/graph/ms 13.75, stage/render/graph_submit 9.44 |
| warm-up | 4.09 | 72.4 | pipelines/compile_ms 72.79, render/thread/ms 71.97, pipelines/front_ms 69.18, render/busy/ms 68.54, main/busy/ms 58.34, stage/render/prepare_assets 33.20, stage/main/update 28.61, stage/main/post_update 22.77, stream/drive/ms 20.63, stage/render/render 19.15, render/graph/ms 15.25, encode//sum 13.71 |
| warm-up | 4.17 | 76.4 | pipelines/compile_ms 1150, pipelines/front_ms 1107, render/thread/ms 111, render/busy/ms 109, main/busy/ms 66.97, stage/render/prepare_assets 59.86, pipelines/driver_ms 43.66, stage/main/update 39.00, stage/render/render 31.21, render/graph/ms 28.01, stage/main/post_update 17.65, stage/render/graph_submit 14.64 |
| warm-up | 4.30 | 130.6 | pipelines/compile_ms 1714, pipelines/front_ms 1640, render/thread/ms 154, render/busy/ms 152, stage/render/prepare_assets 82.18, pipelines/driver_ms 74.43, main/busy/ms 62.70, stage/main/update 37.35, stage/render/prepare_bind_groups 31.87, stage/render/render 29.93, render/graph/ms 26.57, stage/main/post_update 16.71 |
| warm-up | 4.37 | 69.8 | pipelines/compile_ms 2495, pipelines/front_ms 2378, pipelines/wait_ms 277, render/thread/ms 154, render/busy/ms 151, pipelines/driver_ms 116, stage/render/prepare_assets 73.03, main/busy/ms 67.97, stage/render/prepare_bind_groups 38.28, stage/main/post_update 31.15, stage/main/update 23.70, stage/render/render 17.60 |
| warm-up | 4.48 | 112.3 | pipelines/compile_ms 2189, pipelines/front_ms 2095, pipelines/wait_ms 522, render/thread/ms 147, render/busy/ms 144, pipelines/driver_ms 94.26, main/busy/ms 82.29, stage/render/prepare_assets 51.15, stage/render/prepare_write_phase_buffers 35.16, stage/main/update 30.08, stage/main/post_update 29.38, encode//sum 21.08 |
| warm-up | 4.56 | 85.9 | pipelines/compile_ms 1611, pipelines/front_ms 1552, pipelines/wait_ms 617, render/thread/ms 166, render/busy/ms 164, main/busy/ms 76.63, pipelines/driver_ms 59.13, stage/render/prepare_write_phase_buffers 46.83, stage/render/prepare_assets 41.28, encode//sum 35.73, stage/main/post_update 32.13, stage/render/prepare_resources 30.33 |
| warm-up | 4.64 | 80.3 | pipelines/compile_ms 2940, pipelines/front_ms 2846, pipelines/wait_ms 1856, render/thread/ms 134, render/busy/ms 131, pipelines/driver_ms 93.59, main/busy/ms 72.06, stage/render/prepare_assets 46.58, encode//sum 44.13, stage/main/update 31.27, stage/render/prepare_resources 29.20, stage/render/render 26.22 |
| warm-up | 4.68 | 40.6 | pipelines/compile_ms 2014, pipelines/front_ms 1956, pipelines/wait_ms 1641, render/thread/ms 82.88, render/busy/ms 80.79, main/busy/ms 80.19, stage/main/update 64.50, pipelines/driver_ms 57.97, stream/drive/ms 51.16, stage/render/prepare_assets 48.65, encode//sum 29.24, stage/render/render 19.39 |
| warm-up | 4.72 | 38.2 | pipelines/compile_ms 1116, pipelines/front_ms 1078, pipelines/wait_ms 613, main/busy/ms 64.37, stage/main/update 44.23, render/thread/ms 42.66, render/busy/ms 40.98, pipelines/driver_ms 38.11, stream/drive/ms 37.55, stage/render/prepare_assets 20.75, encode//sum 13.87, stage/render/render 10.85 |
| warm-up | 4.88 | 134.6 | pipelines/compile_ms 726, pipelines/front_ms 705, pipelines/wait_ms 437, main/busy/ms 122, stage/main/update 102, render/thread/ms 69.79, render/busy/ms 68.09, stream/drive/ms 68.06, stage/render/prepare_assets 34.84, pipelines/driver_ms 21.24, stage/render/render 16.66, encode//sum 15.64 |
| warm-up | 4.92 | 34.8 | pipelines/compile_ms 1246, pipelines/front_ms 1206, pipelines/wait_ms 440, main/busy/ms 79.95, render/thread/ms 77.28, render/busy/ms 74.65, stage/main/update 59.13, stream/drive/ms 42.00, pipelines/driver_ms 39.92, stage/render/prepare_assets 39.20, stage/render/render 17.78, encode//sum 15.64 |
| warm-up | 4.97 | 51.2 | pipelines/compile_ms 640, pipelines/front_ms 616, main/busy/ms 80.28, stage/main/update 64.38, render/thread/ms 51.74, render/busy/ms 49.29, stream/drive/ms 43.61, pipelines/driver_ms 23.97, stage/render/prepare_assets 21.46, encode//sum 16.19, stage/render/render 15.85, render/graph/ms 12.86 |
| warm-up | 5.01 | 44.8 | pipelines/compile_ms 275, pipelines/front_ms 263, main/busy/ms 69.75, stage/main/update 55.84, render/thread/ms 51.32, render/busy/ms 49.61, stream/drive/ms 35.04, stage/render/render 20.04, encode//sum 18.29, render/graph/ms 17.55, stage/render/prepare_assets 16.09, stage/render/graph_submit 13.69 |
| warm-up | 5.04 | 30.9 | pipelines/compile_ms 317, pipelines/front_ms 306, render/thread/ms 46.07, main/busy/ms 44.30, render/busy/ms 43.60, stage/main/update 28.82, stage/render/render 18.17, encode//sum 17.63, render/graph/ms 15.02, stage/render/prepare_assets 13.50, stream/drive/ms 12.76, stage/render/graph_submit 11.26 |
| warm-up | 5.12 | 32.3 | pipelines/compile_ms 851, pipelines/front_ms 827, render/thread/ms 49.93, render/busy/ms 47.73, main/busy/ms 33.60, pipelines/driver_ms 23.40, stage/render/render 19.88, stage/main/update 19.43, encode//sum 19.16, render/graph/ms 17.35, stage/render/graph_submit 13.74, encode/submit 9.56 |
| warm-up | 5.15 | 28.6 | pipelines/compile_ms 71.87, pipelines/front_ms 67.38, render/thread/ms 55.79, render/busy/ms 53.67, main/busy/ms 41.86, stage/render/render 22.48, stage/main/update 21.56, encode//sum 20.96, render/graph/ms 19.83, stage/render/graph_submit 14.95, stage/main/post_update 14.89, encode/submit 10.36 |
| warm-up | 5.19 | 34.5 | pipelines/compile_ms 256, pipelines/front_ms 248, render/thread/ms 51.05, render/busy/ms 49.42, main/busy/ms 38.95, stage/render/prepare_assets 18.82, encode//sum 16.72, stage/main/post_update 16.54, stage/main/update 16.53, stage/render/render 15.99, render/graph/ms 13.84, stage/render/graph_submit 9.60 |
| warm-up | 5.26 | 46.0 | pipelines/compile_ms 300, pipelines/front_ms 291, main/busy/ms 53.20, render/thread/ms 49.38, render/busy/ms 47.67, stage/main/post_update 37.72, stage/render/render 17.99, stage/render/prepare_assets 17.12, encode//sum 16.80, render/graph/ms 15.76, stage/render/graph_submit 12.21, stage/main/update 10.35 |
| warm-up | 5.34 | 31.6 | pipelines/compile_ms 826, pipelines/front_ms 802, render/thread/ms 51.98, render/busy/ms 49.90, main/busy/ms 47.39, stage/main/update 28.72, pipelines/driver_ms 23.79, stage/render/render 22.53, render/graph/ms 19.79, encode//sum 18.37, stream/drive/ms 18.19, stage/render/graph_submit 14.27 |
| warm-up | 5.39 | 43.2 | pipelines/compile_ms 794, pipelines/front_ms 768, render/thread/ms 63.77, render/busy/ms 61.83, main/busy/ms 47.80, stage/main/update 28.43, stage/render/prepare_assets 26.37, pipelines/driver_ms 25.87, stage/render/render 22.34, render/graph/ms 19.50, encode//sum 18.43, stream/drive/ms 17.60 |
| warm-up | 5.42 | 31.5 | pipelines/compile_ms 753, pipelines/front_ms 729, render/thread/ms 59.91, render/busy/ms 58.53, stage/render/prepare_assets 31.10, main/busy/ms 30.10, pipelines/driver_ms 23.64, encode//sum 21.21, stage/render/render 17.60, render/graph/ms 15.27, stage/main/post_update 14.46, stage/render/graph_submit 11.43 |
| warm-up | 5.47 | 30.8 | pipelines/compile_ms 413, pipelines/front_ms 405, render/thread/ms 41.42, render/busy/ms 40.11, pipelines/wait_ms 37.02, stage/render/prepare_assets 22.66, main/busy/ms 21.12, encode//sum 18.29, stage/render/render 11.95, render/graph/ms 10.13, stage/main/post_update 9.89, stage/render/graph_submit 7.71 |
| warm-up | 5.57 | 28.7 | render/thread/ms 43.78, render/busy/ms 42.55, main/busy/ms 26.17, stage/render/prepare_assets 22.80, encode//sum 17.14, stage/main/update 12.77, stage/render/render 10.93, render/graph/ms 9.21, stage/main/post_update 8.05, stage/render/graph_submit 6.79, stream/drive/ms 6.45, encode/submit 3.82 |
| warm-up | 6.07 | 36.2 | render/thread/ms 44.80, render/busy/ms 42.41, main/busy/ms 25.01, encode//sum 19.51, stage/render/prepare_assets 18.43, stage/render/render 11.27, stage/main/update 10.23, stage/main/post_update 8.94, render/graph/ms 8.36, stage/render/graph_submit 6.21, stage/render/specialize 4.51, encode/finish_wall 4.03 |
| warm-up | 6.29 | 31.8 | pipelines/compile_ms 77.03, pipelines/front_ms 71.11, render/thread/ms 44.14, render/busy/ms 42.66, main/busy/ms 35.38, encode//sum 28.51, stage/render/render 17.62, stage/main/update 17.25, render/graph/ms 15.43, stage/main/post_update 12.04, stage/render/graph_submit 10.53, stage/render/prepare_assets 9.22 |
| warm-up | 6.39 | 35.6 | main/busy/ms 47.03, render/thread/ms 32.03, render/busy/ms 30.49, stage/main/update 28.60, encode//sum 21.98, stream/drive/ms 14.40, stage/render/render 13.47, stage/main/post_update 12.75, render/graph/ms 11.32, stage/render/graph_submit 8.75, stage/render/prepare_assets 7.45, encode/finish_wall 4.65 |
| warm-up | 6.42 | 31.4 | render/thread/ms 38.99, render/busy/ms 37.61, main/busy/ms 34.09, encode//sum 22.42, stage/main/update 17.71, stage/render/prepare_assets 17.54, stage/render/render 11.90, stage/main/post_update 10.64, render/graph/ms 9.94, stage/render/graph_submit 7.49, encode/finish_wall 4.62, stream/drive/ms 4.52 |
| warm-up | 6.60 | 30.4 | render/thread/ms 45.91, render/busy/ms 44.62, main/busy/ms 29.52, encode//sum 24.65, stage/render/prepare_assets 23.74, stage/main/update 14.85, stage/render/render 13.48, render/graph/ms 11.75, stage/main/post_update 10.01, stage/render/graph_submit 9.38, stream/drive/ms 7.55, encode/submit 4.81 |
| warm-up | 6.78 | 33.5 | main/busy/ms 40.70, render/thread/ms 25.74, render/busy/ms 24.47, stage/main/update 21.86, encode//sum 21.39, stream/drive/ms 14.39, stage/main/post_update 13.71, stage/render/render 13.36, stream/spawn/ms 13.13, render/graph/ms 11.67, stage/render/graph_submit 9.33, stage/render/prepare_assets 4.96 |
| measure | 8.13 | 32.7 | encode//sum 135, render/thread/ms 41.17, render/busy/ms 39.83, stage/render/render 29.01, render/graph/ms 27.12, stage/render/graph_submit 21.43, main/busy/ms 17.27, encode/finish_wall 12.84, stage/main/post_update 10.30, encode/resolve_diagnostics/sum 9.54, encode/submit 8.65, stage/render/camera_driver 5.58 |
| measure | 18.18 | 29.7 | main/busy/ms 37.83, encode//sum 28.32, stage/main/update 24.76, render/thread/ms 21.57, stream/drive/ms 20.72, render/busy/ms 20.52, stage/render/render 13.38, render/graph/ms 11.88, stage/main/post_update 10.33, stage/render/graph_submit 9.34, encode/submit 4.96, encode/finish_wall 4.47 |
| measure | 59.05 | 30.0 | render/thread/ms 40.66, render/busy/ms 39.27, main/busy/ms 26.93, encode//sum 26.92, stage/main/post_update 14.42, stage/render/render 14.33, stage/render/prepare_assets 14.12, render/graph/ms 12.62, stage/render/graph_submit 9.08, stage/main/update 6.78, encode/finish_wall 5.23, encode/submit 3.94 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_thr12\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_COMPUTE_THREADS=12
- before the run: GPU 0%, CPU 1%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **77%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.2 W · 72 °C (max 77) · VRAM 5251 MiB · throttled 100% of samples
