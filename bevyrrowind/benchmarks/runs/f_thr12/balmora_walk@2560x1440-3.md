# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5676 frames over 65.6 s · **median 11.32 ms (88 fps)** · mean 11.56 · p95 13.89 · p99 16.34 (1% low 61 fps) · worst 31.26 ms · 39 spikes over 1.5× median · 23.8 stutters/min (work 20.1/min)
- warm-up: 322 frames over 9.0 s, worst 2516.3 ms
- hitches (> 28.3 ms): **3** measured, 40 in warm-up
- timed GPU passes: 7.30 ms of a 11.56 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.32 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.77/min, limit 1/min
- pass worst frame 31.26 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.029 | 2.072 | 2.109 | 0.98 | 0.173 |
| main_opaque_pass_3d | 1.610 | 1.644 | 2.035 | 0.98 | 0.065 |
| early prepass | 0.545 | 0.556 | 1.108 | 0.98 | 0.079 |
| bloom | 0.314 | 0.320 | 0.327 | 0.98 | 0.063 |
| atmosphere_luts | 0.310 | 0.316 | 0.376 | 0.98 | 0.003 |
| water_velocity | 0.278 | 0.284 | 0.353 | 0.98 | 0.002 |
| water_sspr | 0.201 | 0.206 | 0.205 | 0.98 | 0.002 |
| ssao | 0.188 | 0.192 | 0.195 | 0.98 | 0.007 |
| auxiliary_views | 0.154 | 0.157 | 0.550 | 0.98 | 0.146 |
| uniform_allocation | 0.149 | 0.152 | 0.429 | 0.98 | 0.024 |
| atmos_froxel_inject | 0.144 | 0.147 | 0.147 | 0.98 | 0.001 |
| per_view_shadow_pass_cascade1 | 0.143 | 0.572 | 0.644 | 0.25 | 0.042 |
| clouds_trace | 0.140 | 0.143 | 0.233 | 0.98 | 0.001 |
| water_fft | 0.131 | 0.134 | 0.140 | 0.98 | 0.003 |
| upscaling | 0.115 | 0.118 | 0.127 | 0.98 | 0.004 |
| clouds_composite | 0.106 | 0.108 | 0.110 | 0.98 | 0.002 |
| bin_unpacking | 0.100 | 0.102 | 0.126 | 0.98 | 0.018 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.98 | 0.004 |
| main_transmissive_pass_3d | 0.082 | 0.083 | 0.160 | 0.98 | 0.008 |
| clustering | 0.064 | 0.066 | 0.068 | 0.98 | 0.019 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.436 | 0.723 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.98 | 0.001 |
| surface_trail | 0.043 | 0.059 | 0.057 | 0.73 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.030 | 0.042 | 0.98 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.238 | 0.242 | 0.11 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.98 | 0.113 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.98 | 0.004 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| lightprobe_downsampling_second_pass | 0.001 | 0.013 | 0.012 | 0.11 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.038 | 0.049 | 0.01 | 0.013 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.045 | 0.047 | 0.00 | 0.018 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 12.527 | 19.191 | 65.35 | 71104.7 |
| render/thread/ms | 10.295 | 12.470 | 30.24 | 58432.5 |
| render/busy/ms | 9.707 | 11.883 | 29.63 | 55094.5 |
| main/busy/ms | 9.019 | 11.143 | 28.99 | 51194.4 |
| stage/render/render | 6.525 | 8.010 | 26.86 | 37036.6 |
| render/graph/ms | 5.749 | 7.197 | 26.06 | 32631.9 |
| stage/main/post_update | 5.599 | 7.375 | 9.76 | 31781.4 |
| stage/render/graph_submit | 4.581 | 5.740 | 24.52 | 26001.0 |
| encode/submit | 2.407 | 3.213 | 6.03 | 13664.3 |
| encode/finish_wall | 2.245 | 2.875 | 21.47 | 12743.1 |
| stage/main/update | 2.175 | 2.620 | 22.60 | 12347.1 |
| stage/render/camera_driver | 1.113 | 1.522 | 3.85 | 6317.1 |
| stage/render/prepare_write_phase_buffers | 0.790 | 1.083 | 2.49 | 4482.6 |
| bigspace/total/ms | 0.633 | 1.799 | 3.43 | 3592.7 |
| stage/render/prepare_assets | 0.571 | 0.735 | 10.44 | 3242.6 |
| render/present/ms | 0.496 | 0.605 | 5.16 | 2813.4 |
| stage/render/prepare_resources | 0.463 | 0.557 | 1.08 | 2626.3 |
| stage/main/first | 0.398 | 0.521 | 5.25 | 2261.4 |
| stage/render/prepare_meshes | 0.393 | 0.508 | 1.59 | 2229.5 |
| stage/main/pre_update | 0.329 | 0.486 | 1.64 | 1870.0 |
| terrain/update/ms | 0.326 | 0.460 | 1.62 | 1847.7 |
| terrain/select/ms | 0.290 | 0.415 | 0.80 | 1648.1 |
| stage/render/specialize | 0.281 | 0.413 | 2.59 | 1592.9 |
| controller/feed/ms | 0.279 | 0.356 | 0.72 | 1582.2 |
| stage/render/prepare_bind_groups | 0.263 | 0.449 | 2.63 | 1494.4 |
| stream/drive/ms | 0.252 | 0.334 | 20.70 | 1428.7 |
| bigspace/low_precision/ms | 0.241 | 0.704 | 2.83 | 1366.9 |
| bigspace/high_precision/ms | 0.235 | 0.781 | 3.12 | 1332.5 |
| stage/main/fixed_main_loop | 0.223 | 0.362 | 1.15 | 1264.2 |
| stage/render/queue | 0.211 | 0.357 | 2.53 | 1197.9 |
| far/select/ms | 0.202 | 0.276 | 0.94 | 1147.3 |
| stream/tick/ms | 0.194 | 0.265 | 0.79 | 1100.6 |
| anim/evaluate/ms | 0.187 | 0.283 | 2.36 | 1063.8 |
| stage/render/create_views | 0.183 | 0.242 | 0.93 | 1038.1 |
| stage/render/prepare_flush | 0.179 | 0.343 | 0.88 | 1015.5 |
| stage/render/extract_commands | 0.176 | 0.210 | 0.42 | 999.0 |
| stage/render/prepare_batch_phases | 0.157 | 0.216 | 0.50 | 890.9 |
| anim/evaluate_cpu/ms | 0.151 | 0.195 | 0.68 | 855.3 |
| controller/move/ms | 0.138 | 0.250 | 0.52 | 782.3 |
| stage/render/prepare_views | 0.134 | 0.168 | 0.43 | 759.7 |
| stage/main/last | 0.121 | 0.195 | 1.67 | 686.2 |
| render/acquire/ms | 0.092 | 0.118 | 0.39 | 524.6 |
| stage/render/phase_sort | 0.078 | 0.105 | 0.31 | 443.9 |
| pipelines/compile_ms | 0.053 | 0.000 | 126.55 | 298.9 |
| pipelines/front_ms | 0.051 | 0.000 | 122.66 | 286.8 |
| physics/step/ms | 0.050 | 0.085 | 0.73 | 283.7 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.075 | 0.33 | 276.0 |
| physics/insert/ms | 0.036 | 0.389 | 0.90 | 206.8 |
| encode/resolve_diagnostics/sum | 0.035 | 0.047 | 0.31 | 197.2 |
| stage/render/cleanup | 0.033 | 0.110 | 0.76 | 187.4 |
| weather/map/ms | 0.027 | 0.034 | 0.37 | 151.2 |
| encode/camera_driver_span_begin/sum | 0.017 | 0.024 | 0.40 | 98.7 |
| encode/camera_driver_span_end/sum | 0.013 | 0.021 | 0.34 | 73.1 |
| anim/apply/ms | 0.012 | 0.021 | 0.21 | 68.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.14 | 61.3 |
| controller/avoid/ms | 0.011 | 0.014 | 0.05 | 60.0 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 39.6 |
| stream/despawn/ms | 0.005 | 0.000 | 0.94 | 29.2 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 28.1 |
| script/ui/ms | 0.005 | 0.006 | 0.02 | 26.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.4 |
| controller/write/ms | 0.004 | 0.005 | 0.05 | 20.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 14.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.21 | 12.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.06 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.2 |
| stage/render/prepare | 0.001 | 0.002 | 0.04 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.13 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.12 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 30 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1285 | 2262 | 641 |
| crowd/c0/meshes/count | count | 353 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5555 |
| crowd/c1/meshes/count | count | 1179 | 1295 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 237 |
| crowd/joints_extracted/count | count | 4621 | 10.2 k | 6494 |
| crowd/joints_uploaded/count | count | 4604 | 6835 | 6494 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.91 | 21.41 | 1.68 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.40 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.34 | 0.01 |
| encode/finish_critical | ms | 1.91 | 21.41 | 1.68 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.31 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.73 | 29.63 | 9.17 |
| gpu/timed/ms | ms | 8.65 | 12.16 | 9.17 |
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
| phases/alpha_mask3d/items | count | 813 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 813 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1291 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3410 | 5131 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1291 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3410 | 5131 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 197 | 4 |
| phases/shadow/bins | count | 4371 | 5559 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.1 k | 12.5 k | 5432 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 190 | 233 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 299 | 300 | 300 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 551 | 6 |
| preprocess/bind_groups_created | count | 13 | 551 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 210 | 123 |
| process/cpu_usage | % | 28.37 | 47.09 | 20.83 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.37 | 55.66 | 31.20 |
| system/mem_usage | % | 50.47 | 51.02 | 50.96 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 68 | 244 | 54 |
| views/all/bins | count | 7006 | 8511 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 24.9 k | 13.0 k |
| views/all/pipelines | count | 235 | 278 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.9 k | 22.4 k | 8159 |
| views/cascade/batch_sets | count | 10 | 33 | 4 |
| views/cascade/bins | count | 2225 | 2879 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7664 | 9657 | 5432 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 533 | 9551 | 239 |
| views/lamp_face/batch_sets | count | 0 | 187 | 0 |
| views/lamp_face/bins | count | 2142 | 2854 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2400 | 3152 | 0 |
| views/lamp_face/pipelines | count | 165 | 202 | 0 |
| views/lamp_face/work_items | count | 0 | 2848 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2879 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8434 | 13.0 k | 7612 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8598 | 13.2 k | 7920 |
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
| anim/bones_written/count | count | 67.51 | 286 | 383.2 k |
| anim/events/count | count | 0.09 | 3 | 523 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.85 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.82 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.82 | 1312 | 4675 |
| encode//count | count | 39.54 | 272 | 224.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5676 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5676 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5676 |
| physics/inserts | count | 0.40 | 91 | 2258 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.69 | 2 | 3938 |
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
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5964 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1308 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.3 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2516.3 | main/busy/ms 2490, stage/main/post_update 2386, stage/main/update 97.42, surface/advance_cpu_ms 67.18, stage/main/pre_update 2.51, stage/main/last 0.97, surface/cell_upload_ms 0.72, stage/main/first 0.58, stream/drive/ms 0.56, stage/main/fixed_main_loop 0.43, weather/map/ms 0.22, stream/tick/ms 0.14 |
| warm-up | 3.22 | 704.6 | main/busy/ms 2622, stage/main/post_update 2409, render/thread/ms 674, render/busy/ms 508, stage/render/prepare_assets 472, stage/main/update 202, stage/render/prepare_views 165, render/acquire/ms 164, surface/advance_cpu_ms 67.18, stage/render/prepare_resources 21.72, encode//sum 13.15, stage/render/render 10.13 |
| warm-up | 3.30 | 83.3 | render/thread/ms 740, render/busy/ms 573, stage/render/prepare_assets 518, stage/render/prepare_views 165, render/acquire/ms 164, main/busy/ms 154, pipelines/compile_ms 136, pipelines/front_ms 122, stage/main/update 109, stage/main/post_update 34.36, stage/render/prepare_resources 25.27, stage/render/render 23.17 |
| warm-up | 3.33 | 28.6 | pipelines/compile_ms 277, pipelines/front_ms 250, render/thread/ms 90.66, pipelines/wait_ms 86.30, render/busy/ms 82.11, main/busy/ms 75.12, stage/main/post_update 57.52, stage/render/prepare_assets 46.37, stage/render/render 30.40, pipelines/driver_ms 26.36, terrain/update/ms 23.49, render/graph/ms 21.74 |
| warm-up | 3.39 | 61.4 | pipelines/wait_ms 968, pipelines/compile_ms 498, pipelines/front_ms 464, main/busy/ms 72.48, stage/main/post_update 61.12, render/thread/ms 55.96, render/busy/ms 48.01, pipelines/driver_ms 33.89, stage/render/prepare_assets 22.67, stage/render/render 21.38, render/graph/ms 13.15, terrain/update/ms 12.53 |
| warm-up | 3.50 | 104.7 | pipelines/wait_ms 3974, pipelines/compile_ms 1529, pipelines/front_ms 1457, render/thread/ms 133, render/busy/ms 131, pipelines/driver_ms 71.55, stage/render/prepare_resources 67.96, main/busy/ms 58.95, stage/render/prepare_assets 41.66, stage/main/post_update 31.18, stage/main/pre_update 20.18, terrain/update/ms 17.76 |
| warm-up | 3.54 | 45.6 | pipelines/wait_ms 3335, pipelines/compile_ms 1564, pipelines/front_ms 1500, render/thread/ms 142, render/busy/ms 139, main/busy/ms 117, stage/render/prepare_resources 73.40, pipelines/driver_ms 64.21, stage/main/update 61.44, stage/render/render 31.51, encode//sum 31.06, render/graph/ms 28.45 |
| warm-up | 3.69 | 140.7 | pipelines/compile_ms 1282, pipelines/front_ms 1178, pipelines/wait_ms 255, render/thread/ms 141, render/busy/ms 138, pipelines/driver_ms 104, main/busy/ms 101, stage/main/update 67.78, stage/render/prepare_assets 66.96, stage/render/render 59.67, render/graph/ms 56.93, stage/render/graph_submit 40.36 |
| warm-up | 3.74 | 30.0 | pipelines/compile_ms 1331, pipelines/front_ms 1241, pipelines/wait_ms 317, pipelines/driver_ms 89.97, render/thread/ms 51.25, render/busy/ms 47.59, stage/render/prepare_assets 32.00, main/busy/ms 31.08, stage/main/post_update 16.71, stage/render/render 11.71, stage/main/update 9.68, terrain/update/ms 9.65 |
| warm-up | 3.86 | 82.4 | pipelines/compile_ms 328, pipelines/front_ms 317, main/busy/ms 53.42, render/thread/ms 42.92, render/busy/ms 41.34, stage/main/update 41.09, stream/drive/ms 36.98, stage/render/prepare_assets 28.04, pipelines/driver_ms 11.25, stage/render/render 8.87, stage/main/post_update 7.77, encode//sum 7.24 |
| warm-up | 3.93 | 48.2 | pipelines/compile_ms 772, pipelines/front_ms 737, render/thread/ms 63.56, render/busy/ms 61.25, pipelines/driver_ms 35.50, stage/render/prepare_assets 30.17, stage/render/render 26.71, main/busy/ms 24.43, render/graph/ms 24.25, stage/render/graph_submit 12.45, stage/main/update 12.35, encode/submit 7.73 |
| warm-up | 3.98 | 44.4 | pipelines/compile_ms 490, pipelines/front_ms 474, render/thread/ms 84.28, render/busy/ms 81.83, main/busy/ms 48.69, stage/render/prepare_assets 39.98, stage/render/render 36.26, render/graph/ms 33.12, stage/main/post_update 21.46, stage/main/update 20.78, stage/render/graph_submit 16.22, pipelines/driver_ms 15.50 |
| warm-up | 4.04 | 65.4 | pipelines/compile_ms 306, pipelines/front_ms 295, render/thread/ms 90.46, render/busy/ms 88.05, stage/render/prepare_assets 55.31, main/busy/ms 54.01, stage/render/render 26.03, render/graph/ms 23.02, stage/main/post_update 22.78, stage/main/update 21.70, pipelines/wait_ms 17.24, pipelines/lock_ms 16.74 |
| warm-up | 4.10 | 56.9 | pipelines/compile_ms 1022, pipelines/front_ms 980, render/thread/ms 95.64, render/busy/ms 93.06, main/busy/ms 64.11, stage/render/prepare_assets 62.72, pipelines/driver_ms 41.81, stage/main/update 39.02, stage/render/render 18.89, stage/main/post_update 16.96, render/graph/ms 15.85, stage/render/graph_submit 11.45 |
| warm-up | 4.16 | 60.5 | pipelines/compile_ms 1466, pipelines/front_ms 1412, render/thread/ms 86.50, render/busy/ms 82.75, main/busy/ms 57.13, pipelines/driver_ms 54.48, stage/render/prepare_assets 47.83, stage/main/update 34.59, pipelines/wait_ms 29.71, stage/render/render 21.15, render/graph/ms 16.82, stage/main/post_update 15.33 |
| warm-up | 4.22 | 59.5 | pipelines/compile_ms 913, pipelines/front_ms 862, render/thread/ms 99.54, render/busy/ms 95.90, main/busy/ms 73.29, pipelines/wait_ms 60.04, pipelines/driver_ms 51.10, encode//sum 42.15, stage/main/update 36.93, stage/render/render 34.48, render/graph/ms 30.40, stage/main/post_update 28.86 |
| warm-up | 4.27 | 51.7 | pipelines/compile_ms 522, pipelines/front_ms 482, pipelines/wait_ms 171, render/thread/ms 100, render/busy/ms 98.13, main/busy/ms 73.79, encode//sum 48.81, pipelines/driver_ms 39.62, stage/main/update 37.10, stage/render/render 31.82, stage/main/post_update 29.57, render/graph/ms 28.89 |
| warm-up | 4.36 | 85.5 | pipelines/compile_ms 1548, pipelines/front_ms 1499, pipelines/wait_ms 1086, render/thread/ms 122, render/busy/ms 120, encode//sum 82.36, main/busy/ms 70.28, pipelines/driver_ms 48.66, stage/render/prepare_assets 44.75, stage/main/update 42.15, stage/render/render 29.99, render/graph/ms 27.04 |
| warm-up | 4.44 | 79.2 | pipelines/compile_ms 2563, pipelines/front_ms 2490, pipelines/wait_ms 1433, render/thread/ms 140, render/busy/ms 137, main/busy/ms 95.34, encode//sum 75.57, pipelines/driver_ms 72.81, stage/main/update 55.77, stage/render/prepare_assets 44.69, stage/render/prepare_resources 42.58, stream/drive/ms 27.79 |
| warm-up | 4.47 | 33.5 | pipelines/compile_ms 1314, pipelines/front_ms 1281, pipelines/wait_ms 579, main/busy/ms 95.67, render/thread/ms 84.03, render/busy/ms 81.57, stage/main/update 67.28, stage/render/prepare_assets 36.59, pipelines/driver_ms 32.41, stream/drive/ms 27.09, stage/render/prepare_resources 21.59, stage/main/post_update 13.78 |
| warm-up | 4.61 | 141.9 | pipelines/compile_ms 2523, pipelines/front_ms 2458, pipelines/wait_ms 1183, main/busy/ms 104, render/thread/ms 99.32, render/busy/ms 97.48, stage/main/update 84.37, pipelines/driver_ms 64.72, stream/drive/ms 47.60, stage/render/prepare_assets 45.87, stage/render/prepare_resources 21.78, encode//sum 12.62 |
| warm-up | 4.68 | 66.4 | pipelines/compile_ms 2549, pipelines/front_ms 2466, pipelines/wait_ms 1102, render/thread/ms 139, render/busy/ms 134, main/busy/ms 104, pipelines/driver_ms 83.36, stage/main/update 55.97, stage/render/render 41.93, stage/render/prepare_assets 38.62, stage/main/post_update 37.56, render/graph/ms 37.16 |
| warm-up | 4.74 | 62.2 | pipelines/compile_ms 370, pipelines/front_ms 342, main/busy/ms 130, render/thread/ms 104, render/busy/ms 97.73, stage/main/update 83.01, stream/drive/ms 59.95, stage/render/render 46.71, render/graph/ms 41.45, stage/main/post_update 36.17, pipelines/driver_ms 28.04, stage/render/graph_submit 26.71 |
| warm-up | 4.83 | 91.3 | pipelines/compile_ms 1621, pipelines/front_ms 1581, main/busy/ms 130, stage/main/update 108, stream/drive/ms 94.54, render/thread/ms 79.41, render/busy/ms 77.46, stage/render/prepare_assets 47.47, pipelines/driver_ms 39.20, stage/render/render 17.44, encode//sum 15.67, render/graph/ms 14.90 |
| warm-up | 4.89 | 56.3 | pipelines/compile_ms 1538, pipelines/front_ms 1501, main/busy/ms 78.26, render/thread/ms 64.97, render/busy/ms 63.26, stage/main/update 56.14, stage/render/prepare_assets 42.89, stream/drive/ms 41.01, pipelines/driver_ms 37.85, encode//sum 15.68, stage/main/post_update 14.00, stage/render/render 11.23 |
| warm-up | 4.98 | 61.1 | pipelines/compile_ms 225, pipelines/front_ms 218, main/busy/ms 60.44, stage/main/update 46.12, render/thread/ms 43.56, render/busy/ms 41.98, stream/drive/ms 29.82, stage/render/prepare_assets 16.44, encode//sum 14.26, stage/render/render 13.12, render/graph/ms 11.00, stage/render/graph_submit 8.41 |
| warm-up | 5.09 | 33.2 | pipelines/compile_ms 950, pipelines/front_ms 920, render/thread/ms 53.99, render/busy/ms 51.66, main/busy/ms 35.88, pipelines/driver_ms 30.57, stage/render/render 18.63, encode//sum 16.90, stage/main/update 16.18, render/graph/ms 15.83, stage/main/post_update 13.10, stage/render/graph_submit 11.61 |
| warm-up | 5.15 | 37.6 | pipelines/compile_ms 388, pipelines/front_ms 375, render/thread/ms 49.42, render/busy/ms 47.68, main/busy/ms 42.78, stage/main/update 22.93, stage/render/render 16.75, encode//sum 16.24, stage/render/prepare_assets 16.14, render/graph/ms 14.49, stage/main/post_update 14.28, pipelines/driver_ms 12.92 |
| warm-up | 5.18 | 31.1 | pipelines/compile_ms 608, pipelines/front_ms 590, render/thread/ms 55.55, render/busy/ms 53.63, main/busy/ms 41.15, stage/render/render 22.19, stage/main/update 20.97, stage/render/prepare_assets 19.86, render/graph/ms 19.58, pipelines/driver_ms 18.12, encode//sum 16.37, stage/render/graph_submit 15.51 |
| warm-up | 5.21 | 32.5 | pipelines/compile_ms 490, pipelines/front_ms 475, render/thread/ms 52.40, render/busy/ms 50.61, main/busy/ms 27.23, stage/render/prepare_assets 20.43, stage/render/render 19.79, render/graph/ms 17.42, pipelines/driver_ms 14.98, stage/render/graph_submit 14.93, encode//sum 13.32, encode/finish_wall 10.54 |
| warm-up | 5.25 | 33.5 | pipelines/compile_ms 121, pipelines/front_ms 114, render/thread/ms 57.21, render/busy/ms 55.60, main/busy/ms 42.27, stage/render/render 21.43, render/graph/ms 19.25, stage/main/post_update 17.64, stage/render/graph_submit 16.12, stage/main/update 15.72, encode//sum 14.49, stage/render/prepare_assets 12.86 |
| warm-up | 5.33 | 38.1 | pipelines/compile_ms 1303, pipelines/front_ms 1257, main/busy/ms 61.96, render/thread/ms 48.08, pipelines/driver_ms 46.46, render/busy/ms 45.07, stage/main/update 31.77, stage/render/render 27.26, stage/main/post_update 24.55, render/graph/ms 23.70, encode//sum 22.11, stage/render/graph_submit 17.10 |
| warm-up | 5.37 | 35.6 | pipelines/compile_ms 1769, pipelines/front_ms 1712, pipelines/driver_ms 57.45, render/thread/ms 48.68, render/busy/ms 46.75, main/busy/ms 44.97, stage/main/post_update 25.52, stage/render/prepare_assets 17.43, stage/render/render 16.81, encode//sum 15.64, render/graph/ms 14.34, stage/main/update 13.63 |
| warm-up | 5.96 | 29.3 | render/thread/ms 48.35, render/busy/ms 46.56, main/busy/ms 23.13, encode//sum 21.39, stage/render/prepare_assets 14.58, stage/render/render 13.88, render/graph/ms 11.56, stage/main/update 9.68, stage/render/graph_submit 8.74, stage/main/post_update 8.69, stage/render/specialize 6.68, encode/submit 5.00 |
| warm-up | 6.03 | 42.4 | render/thread/ms 54.18, render/busy/ms 52.80, main/busy/ms 33.31, stage/render/prepare_assets 28.17, encode//sum 21.47, stage/main/post_update 13.51, stage/main/update 13.03, stage/render/render 10.77, render/graph/ms 8.85, stage/render/graph_submit 6.61, stream/drive/ms 4.42, stage/render/specialize 4.20 |
| warm-up | 6.08 | 28.6 | render/thread/ms 49.59, render/busy/ms 48.40, stage/render/prepare_assets 26.22, main/busy/ms 24.94, encode//sum 21.55, stage/render/render 10.91, stage/main/update 10.90, render/graph/ms 9.20, stage/main/post_update 9.06, stage/render/graph_submit 6.67, encode/finish_wall 3.59, stream/drive/ms 3.41 |
| warm-up | 6.14 | 31.5 | render/thread/ms 48.51, render/busy/ms 47.29, main/busy/ms 30.96, stage/render/prepare_assets 26.14, encode//sum 19.10, stage/main/update 16.05, stage/main/post_update 10.57, stage/render/render 9.43, render/graph/ms 7.72, stage/render/graph_submit 5.73, stream/drive/ms 3.72, encode/finish_wall 3.21 |
| warm-up | 6.32 | 30.3 | pipelines/compile_ms 78.11, pipelines/front_ms 74.44, render/thread/ms 39.50, render/busy/ms 38.47, main/busy/ms 30.59, stage/render/prepare_assets 20.29, encode//sum 19.63, stage/main/update 16.02, stage/main/post_update 9.43, stage/render/render 9.06, render/graph/ms 7.50, stage/render/graph_submit 5.28 |
| warm-up | 6.42 | 32.5 | main/busy/ms 44.75, render/thread/ms 35.47, render/busy/ms 33.86, stage/main/update 31.63, encode//sum 21.97, stream/drive/ms 16.90, stage/render/prepare_assets 13.89, stage/render/render 11.64, render/graph/ms 9.33, stage/main/post_update 8.27, stage/render/graph_submit 6.94, encode/finish_wall 3.78 |
| warm-up | 6.58 | 29.1 | render/thread/ms 45.37, render/busy/ms 44.27, main/busy/ms 26.36, stage/render/prepare_assets 26.11, encode//sum 20.06, stage/main/update 12.68, stage/render/render 11.31, render/graph/ms 9.84, stage/main/post_update 9.73, stage/render/graph_submit 7.86, encode/finish_wall 4.08, encode/submit 3.85 |
| measure | 1.40 | 31.3 | encode//sum 72.03, render/thread/ms 40.85, render/busy/ms 39.75, stage/render/render 33.80, render/graph/ms 32.23, stage/render/graph_submit 29.18, encode/finish_wall 23.37, main/busy/ms 18.79, stage/main/post_update 12.18, encode/submit 5.91, stage/main/update 4.18, stage/render/camera_driver 2.94 |
| measure | 18.18 | 31.1 | main/busy/ms 39.27, encode//sum 32.87, stage/main/update 24.56, render/thread/ms 23.18, render/busy/ms 22.09, stream/drive/ms 20.93, stage/render/render 14.03, render/graph/ms 12.53, stage/main/post_update 12.07, stage/render/graph_submit 9.14, encode/submit 5.19, encode/finish_wall 4.03 |
| measure | 59.04 | 28.3 | render/thread/ms 40.60, render/busy/ms 39.44, main/busy/ms 27.46, encode//sum 24.44, stage/render/prepare_assets 16.17, stage/main/post_update 14.82, stage/render/render 12.99, render/graph/ms 11.39, stage/render/graph_submit 8.22, stage/main/update 7.86, encode/submit 4.19, encode/finish_wall 4.12 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_thr12\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_COMPUTE_THREADS=12
- before the run: GPU 0%, CPU 5%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 57%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.5 W · 73 °C (max 78) · VRAM 5248 MiB · throttled 100% of samples
