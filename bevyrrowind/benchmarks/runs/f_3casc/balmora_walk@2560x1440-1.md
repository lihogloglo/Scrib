# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5498 frames over 65.6 s · **median 11.66 ms (86 fps)** · mean 11.94 · p95 14.42 · p99 16.88 (1% low 59 fps) · worst 32.32 ms · 38 spikes over 1.5× median · 28.3 stutters/min (work 27.4/min)
- warm-up: 325 frames over 9.1 s, worst 2527.6 ms
- hitches (> 29.2 ms): **2** measured, 35 in warm-up
- timed GPU passes: 7.37 ms of a 11.94 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.66 ms, limit 11 ms
- **FAIL** frame_ms: 31 stutters (> 1.5x moving median of 60) in 1.09 min = 28.33/min, limit 1/min
- pass worst frame 32.32 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.040 | 2.082 | 2.276 | 0.98 | 0.179 |
| main_opaque_pass_3d | 1.645 | 1.679 | 2.153 | 0.98 | 0.066 |
| early prepass | 0.567 | 0.579 | 1.097 | 0.98 | 0.082 |
| atmosphere_luts | 0.322 | 0.328 | 0.498 | 0.98 | 0.004 |
| bloom | 0.315 | 0.322 | 0.327 | 0.98 | 0.065 |
| water_velocity | 0.279 | 0.285 | 0.354 | 0.98 | 0.002 |
| water_sspr | 0.202 | 0.207 | 0.206 | 0.98 | 0.002 |
| ssao | 0.199 | 0.203 | 0.209 | 0.98 | 0.007 |
| atmos_froxel_inject | 0.147 | 0.150 | 0.150 | 0.98 | 0.001 |
| clouds_trace | 0.140 | 0.143 | 0.236 | 0.98 | 0.001 |
| auxiliary_views | 0.136 | 0.139 | 0.510 | 0.98 | 0.135 |
| water_fft | 0.134 | 0.136 | 0.140 | 0.98 | 0.003 |
| uniform_allocation | 0.127 | 0.129 | 0.179 | 0.98 | 0.025 |
| upscaling | 0.116 | 0.118 | 0.130 | 0.98 | 0.004 |
| per_view_shadow_pass_cascade2 | 0.111 | 0.459 | 0.524 | 0.24 | 0.037 |
| clouds_composite | 0.105 | 0.108 | 0.110 | 0.98 | 0.002 |
| bin_unpacking | 0.099 | 0.101 | 0.137 | 0.98 | 0.018 |
| tonemapping | 0.085 | 0.087 | 0.090 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.084 | 0.085 | 0.165 | 0.98 | 0.009 |
| clustering | 0.065 | 0.066 | 0.068 | 0.98 | 0.019 |
| render_sky | 0.051 | 0.053 | 0.053 | 0.98 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.431 | 0.560 | 0.11 | 0.001 |
| surface_trail | 0.044 | 0.058 | 0.057 | 0.75 | 0.002 |
| per_view_shadow_pass_cascade1 | 0.038 | 0.156 | 0.173 | 0.24 | 0.036 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.031 | 0.047 | 0.98 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.98 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.98 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.222 | 0.233 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.027 | 0.98 | 0.118 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.012 | 0.022 | 0.98 | 0.005 |
| atmosphere_environment | 0.011 | 0.011 | 0.012 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.017 | 0.017 | 0.00 | 0.011 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 11.253 | 16.691 | 45.54 | 61870.4 |
| render/thread/ms | 10.635 | 12.978 | 24.07 | 58469.3 |
| render/busy/ms | 10.047 | 12.376 | 23.44 | 55239.6 |
| main/busy/ms | 9.573 | 11.835 | 28.56 | 52630.4 |
| stage/render/render | 6.748 | 8.294 | 15.17 | 37102.9 |
| stage/main/post_update | 6.069 | 7.935 | 12.95 | 33367.0 |
| render/graph/ms | 5.966 | 7.448 | 14.28 | 32803.2 |
| stage/render/graph_submit | 4.770 | 5.966 | 10.80 | 26222.8 |
| encode/submit | 2.419 | 3.274 | 7.26 | 13299.4 |
| encode/finish_wall | 2.416 | 3.121 | 7.47 | 13281.3 |
| stage/main/update | 2.213 | 2.705 | 22.27 | 12166.0 |
| stage/render/camera_driver | 1.140 | 1.552 | 3.59 | 6269.1 |
| stage/render/prepare_write_phase_buffers | 0.775 | 1.107 | 2.68 | 4258.7 |
| bigspace/total/ms | 0.614 | 1.964 | 4.58 | 3376.3 |
| stage/render/prepare_assets | 0.579 | 0.748 | 9.34 | 3185.2 |
| render/present/ms | 0.493 | 0.627 | 2.56 | 2712.1 |
| stage/render/prepare_resources | 0.476 | 0.596 | 1.22 | 2615.2 |
| stage/main/first | 0.412 | 0.541 | 5.40 | 2263.7 |
| stage/render/prepare_meshes | 0.407 | 0.531 | 1.89 | 2237.7 |
| stage/main/pre_update | 0.337 | 0.505 | 1.90 | 1853.9 |
| terrain/update/ms | 0.318 | 0.430 | 2.08 | 1747.7 |
| stage/render/specialize | 0.306 | 0.459 | 3.43 | 1682.7 |
| stage/render/prepare_bind_groups | 0.286 | 0.504 | 2.32 | 1573.4 |
| terrain/select/ms | 0.284 | 0.388 | 0.98 | 1561.0 |
| controller/feed/ms | 0.278 | 0.360 | 0.75 | 1528.2 |
| bigspace/low_precision/ms | 0.255 | 0.831 | 2.84 | 1402.3 |
| stream/drive/ms | 0.254 | 0.353 | 20.50 | 1399.2 |
| stage/main/fixed_main_loop | 0.236 | 0.383 | 1.10 | 1299.3 |
| stage/render/queue | 0.235 | 0.411 | 3.64 | 1294.0 |
| bigspace/high_precision/ms | 0.203 | 0.839 | 2.94 | 1115.2 |
| anim/evaluate/ms | 0.200 | 0.353 | 2.59 | 1100.1 |
| stream/tick/ms | 0.196 | 0.265 | 0.79 | 1076.0 |
| far/select/ms | 0.194 | 0.267 | 0.71 | 1064.6 |
| stage/render/create_views | 0.190 | 0.249 | 0.82 | 1043.2 |
| stage/render/prepare_flush | 0.183 | 0.377 | 0.99 | 1004.4 |
| stage/render/extract_commands | 0.179 | 0.216 | 0.88 | 985.3 |
| stage/render/prepare_batch_phases | 0.162 | 0.228 | 0.53 | 892.6 |
| anim/evaluate_cpu/ms | 0.144 | 0.188 | 0.53 | 793.9 |
| stage/render/prepare_views | 0.137 | 0.177 | 0.65 | 752.4 |
| controller/move/ms | 0.125 | 0.206 | 0.59 | 686.7 |
| stage/main/last | 0.124 | 0.198 | 1.63 | 680.2 |
| render/acquire/ms | 0.094 | 0.123 | 0.62 | 517.6 |
| stage/render/phase_sort | 0.084 | 0.117 | 0.30 | 462.9 |
| pipelines/compile_ms | 0.053 | 0.000 | 119.12 | 291.9 |
| physics/step/ms | 0.052 | 0.087 | 0.57 | 284.0 |
| pipelines/front_ms | 0.051 | 0.000 | 115.26 | 280.0 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.077 | 0.31 | 269.7 |
| physics/insert/ms | 0.038 | 0.394 | 0.95 | 208.7 |
| stage/render/cleanup | 0.034 | 0.116 | 0.66 | 189.3 |
| encode/resolve_diagnostics/sum | 0.033 | 0.046 | 0.79 | 179.8 |
| weather/map/ms | 0.026 | 0.034 | 0.25 | 143.5 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.023 | 0.32 | 85.5 |
| anim/apply/ms | 0.012 | 0.022 | 0.25 | 64.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.021 | 0.29 | 62.2 |
| weather/eval/ms | 0.011 | 0.014 | 0.17 | 59.3 |
| controller/avoid/ms | 0.010 | 0.013 | 0.24 | 56.1 |
| script/update/ms | 0.007 | 0.009 | 0.22 | 39.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.09 | 29.5 |
| script/input/ms | 0.005 | 0.006 | 0.27 | 27.8 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 27.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 25.4 |
| controller/write/ms | 0.003 | 0.004 | 0.19 | 19.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 15.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.03 | 11.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.9 |
| stage/render/prepare | 0.001 | 0.002 | 0.23 | 2.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.19 | 1.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1301 | 2262 | 649 |
| crowd/c0/meshes/count | count | 353 | 646 | 177 |
| crowd/c1/count | count | 146 | 156 | 146 |
| crowd/c1/joints/count | count | 5531 | 6004 | 5515 |
| crowd/c1/meshes/count | count | 1176 | 1308 | 1156 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.3 k | 6528 |
| crowd/joints_uploaded/count | count | 4596 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.97 | 7.34 | 2.21 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.32 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.01 |
| encode/finish_critical | ms | 1.97 | 7.34 | 2.21 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.79 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.07 | 28.56 | 9.68 |
| gpu/timed/ms | ms | 8.67 | 10.74 | 9.68 |
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
| phases/alpha_mask3d/items | count | 824 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 824 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1297 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3432 | 5126 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1297 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3432 | 5126 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 201 | 4 |
| phases/shadow/bins | count | 4848 | 5854 | 2303 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.0 k | 13.5 k | 6081 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 197 | 241 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 298 | 299 | 299 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 555 | 6 |
| preprocess/bind_groups_created | count | 14 | 555 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 126 | 197 | 123 |
| process/cpu_usage | % | 26.44 | 40.87 | 23.08 |
| process/mem_usage | GiB | 5.43 | 5.60 | 5.60 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.97 | 62.45 | 30.64 |
| system/mem_usage | % | 59.06 | 59.54 | 59.50 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 62 | 248 | 54 |
| views/all/bins | count | 7402 | 8754 | 5801 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.1 k | 26.0 k | 13.7 k |
| views/all/pipelines | count | 242 | 286 | 74 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 23.3 k | 8063 |
| views/cascade/batch_sets | count | 7 | 43 | 4 |
| views/cascade/bins | count | 2657 | 3239 | 2303 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8546 | 10.5 k | 6081 |
| views/cascade/pipelines | count | 33 | 44 | 28 |
| views/cascade/work_items | count | 245 | 10.3 k | 145 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2135 | 2807 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2382 | 3119 | 0 |
| views/lamp_face/pipelines | count | 164 | 201 | 0 |
| views/lamp_face/work_items | count | 0 | 2900 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2890 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8513 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8671 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 63.82 | 330 | 350.9 k |
| anim/events/count | count | 0.11 | 3 | 592 |
| assets/Image/added | count | 0.11 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.74 | 152 | 4061 |
| assets/Mesh/removed | count | 0.88 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.94 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.85 | 1485 | 4675 |
| encode//count | count | 39.20 | 272 | 215.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5498 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5498 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5498 |
| physics/inserts | count | 0.41 | 111 | 2262 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.72 | 2 | 3939 |
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
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 23 | 378 |
| stream/upload/bytes | bytes | 6157 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1350 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.2 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2527.6 | main/busy/ms 2502, stage/main/post_update 2399, stage/main/update 96.14, surface/advance_cpu_ms 67.01, stage/main/pre_update 2.70, stage/main/last 1.18, surface/cell_upload_ms 0.84, stream/drive/ms 0.71, stage/main/first 0.54, stage/main/fixed_main_loop 0.45, stream/tick/ms 0.18, weather/map/ms 0.11 |
| warm-up | 3.26 | 735.2 | main/busy/ms 2654, stage/main/post_update 2422, render/thread/ms 707, render/busy/ms 597, stage/render/prepare_assets 552, stage/main/update 219, stage/render/prepare_views 108, render/acquire/ms 108, surface/advance_cpu_ms 67.01, stage/render/create_views 27.30, encode//sum 10.58, stage/render/render 10.25 |
| warm-up | 3.32 | 58.9 | render/thread/ms 748, render/busy/ms 637, stage/render/prepare_assets 574, main/busy/ms 172, stage/main/update 128, stage/render/prepare_views 108, render/acquire/ms 108, pipelines/compile_ms 61.52, pipelines/front_ms 57.89, stage/main/post_update 34.19, stage/render/create_views 27.93, stage/render/render 21.06 |
| warm-up | 3.36 | 37.0 | pipelines/compile_ms 219, pipelines/front_ms 199, pipelines/wait_ms 117, render/thread/ms 72.68, render/busy/ms 57.90, main/busy/ms 51.37, stage/render/render 35.71, stage/main/post_update 34.29, terrain/update/ms 24.35, stage/render/prepare_assets 23.11, pipelines/driver_ms 20.42, render/graph/ms 20.35 |
| warm-up | 3.52 | 145.4 | pipelines/wait_ms 5087, pipelines/compile_ms 1425, pipelines/front_ms 1350, main/busy/ms 159, render/thread/ms 145, render/busy/ms 142, stage/main/update 102, pipelines/driver_ms 75.51, stage/render/prepare_resources 73.56, stage/render/prepare_assets 40.49, stage/main/post_update 35.64, stage/main/pre_update 17.76 |
| warm-up | 3.55 | 30.6 | pipelines/wait_ms 4863, pipelines/compile_ms 1564, pipelines/front_ms 1497, main/busy/ms 199, stage/main/update 157, render/thread/ms 156, render/busy/ms 154, stage/render/prepare_resources 76.09, pipelines/driver_ms 66.89, stage/render/prepare_assets 45.13, stage/main/post_update 32.45, terrain/update/ms 13.84 |
| warm-up | 3.65 | 96.5 | pipelines/compile_ms 1562, pipelines/front_ms 1485, pipelines/wait_ms 590, main/busy/ms 91.54, pipelines/driver_ms 76.95, render/thread/ms 76.74, render/busy/ms 74.26, stage/main/update 64.48, stage/render/prepare_assets 52.52, stage/main/post_update 17.48, stage/render/render 13.28, terrain/update/ms 11.26 |
| warm-up | 3.70 | 31.4 | pipelines/compile_ms 356, pipelines/front_ms 342, pipelines/wait_ms 71.00, render/thread/ms 49.27, render/busy/ms 47.25, main/busy/ms 44.03, stage/render/prepare_assets 27.53, stage/main/update 20.04, stage/main/post_update 16.28, pipelines/driver_ms 14.57, stage/render/render 12.13, terrain/update/ms 9.87 |
| warm-up | 3.83 | 66.8 | pipelines/compile_ms 226, pipelines/front_ms 221, main/busy/ms 50.55, render/thread/ms 46.18, render/busy/ms 44.24, stage/main/update 38.37, stream/drive/ms 34.04, stage/render/prepare_assets 24.27, encode//sum 19.30, stage/render/render 15.25, render/graph/ms 12.87, stage/render/graph_submit 8.58 |
| warm-up | 3.87 | 32.8 | pipelines/compile_ms 786, pipelines/front_ms 761, render/thread/ms 58.30, render/busy/ms 56.22, main/busy/ms 47.78, stage/main/update 37.37, stage/render/prepare_assets 35.68, stream/drive/ms 32.34, pipelines/driver_ms 25.55, stage/render/render 15.92, render/graph/ms 13.24, stage/render/graph_submit 8.45 |
| warm-up | 3.92 | 57.0 | pipelines/compile_ms 829, pipelines/front_ms 793, render/thread/ms 64.05, render/busy/ms 59.37, main/busy/ms 52.60, stage/main/update 38.92, stage/render/prepare_assets 35.95, pipelines/driver_ms 35.50, stream/drive/ms 33.45, stage/render/render 20.12, render/graph/ms 15.01, stage/render/graph_submit 9.59 |
| warm-up | 4.04 | 98.2 | pipelines/compile_ms 434, pipelines/front_ms 419, render/thread/ms 73.23, render/busy/ms 70.45, main/busy/ms 51.57, stage/render/prepare_assets 35.13, stage/main/update 30.60, stage/render/render 25.38, render/graph/ms 21.95, stage/render/graph_submit 17.75, encode//sum 17.52, pipelines/driver_ms 15.54 |
| warm-up | 4.11 | 64.7 | pipelines/compile_ms 1153, pipelines/front_ms 1114, render/thread/ms 98.54, render/busy/ms 94.48, stage/render/prepare_assets 62.88, main/busy/ms 60.89, pipelines/driver_ms 38.70, stage/main/update 36.80, stage/render/render 25.66, render/graph/ms 19.96, encode//sum 16.60, stage/render/graph_submit 15.73 |
| warm-up | 4.21 | 104.6 | pipelines/compile_ms 1227, pipelines/front_ms 1190, render/thread/ms 122, render/busy/ms 93.96, stage/render/prepare_assets 70.48, main/busy/ms 52.32, stage/render/render 43.47, pipelines/driver_ms 37.28, stage/main/update 31.69, render/present/ms 27.81, render/graph/ms 14.08, stage/main/post_update 9.40 |
| warm-up | 4.25 | 39.0 | pipelines/compile_ms 689, pipelines/front_ms 595, render/thread/ms 116, pipelines/driver_ms 93.67, render/busy/ms 85.86, main/busy/ms 68.59, stage/render/render 60.96, stage/render/prepare_assets 42.44, stage/main/update 40.45, render/graph/ms 30.68, render/present/ms 29.60, stage/render/graph_submit 19.40 |
| warm-up | 4.33 | 79.2 | pipelines/compile_ms 1509, pipelines/front_ms 1388, pipelines/driver_ms 121, render/thread/ms 78.99, render/busy/ms 73.82, main/busy/ms 70.46, stage/main/update 41.09, stage/render/render 32.02, stage/render/prepare_assets 26.67, render/graph/ms 26.36, stage/main/post_update 21.22, stage/render/graph_submit 20.32 |
| warm-up | 4.40 | 73.2 | pipelines/compile_ms 1834, pipelines/front_ms 1781, render/thread/ms 112, render/busy/ms 110, pipelines/wait_ms 97.32, main/busy/ms 55.44, pipelines/driver_ms 53.55, stage/render/prepare_resources 35.86, stage/render/prepare_assets 29.74, stage/main/post_update 29.52, encode//sum 24.43, stage/render/render 15.59 |
| warm-up | 4.49 | 90.8 | pipelines/compile_ms 2270, pipelines/front_ms 2182, pipelines/wait_ms 774, render/thread/ms 152, render/busy/ms 150, pipelines/driver_ms 88.46, stage/render/prepare_resources 68.12, main/busy/ms 66.03, encode//sum 30.20, stage/render/prepare_write_phase_buffers 28.23, stage/main/post_update 27.94, stage/render/render 17.18 |
| warm-up | 4.55 | 55.2 | pipelines/compile_ms 2285, pipelines/front_ms 2195, pipelines/wait_ms 1243, render/thread/ms 129, render/busy/ms 127, main/busy/ms 101, pipelines/driver_ms 89.07, encode//sum 72.52, stage/main/update 71.94, stream/drive/ms 52.11, stage/render/prepare_resources 36.12, stage/render/render 30.86 |
| warm-up | 4.64 | 94.8 | pipelines/compile_ms 1780, pipelines/front_ms 1734, pipelines/wait_ms 883, render/thread/ms 92.20, render/busy/ms 90.43, main/busy/ms 88.95, encode//sum 80.33, stage/main/update 69.77, stream/drive/ms 50.31, pipelines/driver_ms 46.55, stage/render/prepare_assets 36.67, stage/render/render 36.05 |
| warm-up | 4.82 | 151.8 | pipelines/compile_ms 1691, pipelines/front_ms 1633, pipelines/wait_ms 572, main/busy/ms 104, render/thread/ms 86.45, render/busy/ms 84.76, stage/main/update 83.14, pipelines/driver_ms 57.49, stage/render/prepare_assets 56.94, stream/drive/ms 46.54, stage/render/render 14.61, encode//sum 12.55 |
| warm-up | 4.85 | 35.5 | pipelines/compile_ms 1535, pipelines/front_ms 1480, pipelines/wait_ms 515, render/thread/ms 99.44, render/busy/ms 97.76, main/busy/ms 87.91, stage/main/update 64.99, stage/render/prepare_assets 55.84, pipelines/driver_ms 55.13, stream/drive/ms 43.97, stage/render/render 25.74, render/graph/ms 23.49 |
| warm-up | 4.93 | 81.8 | pipelines/compile_ms 162, pipelines/front_ms 154, main/busy/ms 84.75, stage/main/update 66.76, render/thread/ms 63.38, render/busy/ms 61.81, stream/drive/ms 44.36, stage/render/render 24.40, stage/render/prepare_assets 22.54, render/graph/ms 22.09, stage/render/graph_submit 18.98, encode//sum 15.04 |
| warm-up | 5.00 | 45.0 | pipelines/compile_ms 969, pipelines/front_ms 939, main/busy/ms 57.81, stage/main/update 40.34, render/thread/ms 36.66, render/busy/ms 34.93, stream/drive/ms 33.37, pipelines/driver_ms 29.73, encode//sum 14.14, stage/render/render 13.34, stage/main/post_update 11.50, render/graph/ms 11.15 |
| warm-up | 5.18 | 49.4 | pipelines/compile_ms 792, pipelines/front_ms 771, render/thread/ms 49.01, render/busy/ms 46.88, main/busy/ms 40.30, stage/main/update 24.28, stage/render/prepare_assets 21.09, pipelines/driver_ms 20.62, stage/render/render 17.58, encode//sum 16.33, render/graph/ms 14.99, stream/drive/ms 12.86 |
| warm-up | 5.24 | 33.1 | pipelines/compile_ms 498, pipelines/front_ms 481, render/thread/ms 56.90, render/busy/ms 55.07, main/busy/ms 31.21, stage/render/render 21.89, render/graph/ms 19.90, encode//sum 18.44, pipelines/driver_ms 16.73, stage/main/post_update 14.44, stage/render/prepare_assets 12.25, stage/render/graph_submit 11.93 |
| warm-up | 5.30 | 38.3 | pipelines/compile_ms 102, pipelines/front_ms 98.03, render/thread/ms 56.35, render/busy/ms 54.93, main/busy/ms 45.52, stage/render/prepare_assets 29.95, pipelines/wait_ms 27.67, stage/main/update 20.36, stage/main/post_update 20.12, encode//sum 18.14, stage/render/render 12.09, render/graph/ms 9.90 |
| warm-up | 5.34 | 36.6 | pipelines/compile_ms 82.86, pipelines/front_ms 79.32, render/thread/ms 69.25, render/busy/ms 67.46, main/busy/ms 57.82, stage/main/update 29.25, stage/render/prepare_assets 26.55, pipelines/wait_ms 26.49, stage/main/post_update 23.34, encode//sum 20.88, stage/render/prepare_resources 16.00, stage/render/render 14.53 |
| warm-up | 5.38 | 40.4 | pipelines/compile_ms 447, pipelines/front_ms 434, render/thread/ms 71.15, render/busy/ms 69.27, main/busy/ms 55.87, stage/render/prepare_assets 31.87, stage/main/update 31.15, encode//sum 19.65, stage/main/post_update 18.77, stage/render/prepare_resources 15.11, stage/render/render 14.30, pipelines/driver_ms 12.89 |
| warm-up | 5.43 | 46.4 | pipelines/compile_ms 2313, pipelines/front_ms 2249, render/thread/ms 77.34, render/busy/ms 75.71, pipelines/driver_ms 63.76, main/busy/ms 59.39, stage/render/prepare_assets 47.97, stage/main/update 32.68, stage/main/post_update 17.74, encode//sum 16.98, stage/render/render 13.62, stream/drive/ms 13.30 |
| warm-up | 5.46 | 32.6 | pipelines/compile_ms 2104, pipelines/front_ms 2047, render/thread/ms 66.84, render/busy/ms 65.23, pipelines/driver_ms 57.00, main/busy/ms 46.95, stage/render/prepare_assets 33.81, stage/main/update 24.63, encode//sum 19.72, stage/render/render 14.33, stage/main/post_update 13.32, stream/drive/ms 13.07 |
| warm-up | 6.03 | 29.5 | render/thread/ms 35.35, render/busy/ms 34.21, main/busy/ms 24.99, encode//sum 15.79, stage/render/prepare_assets 13.91, stage/main/update 10.78, stage/main/post_update 9.32, stage/render/render 9.00, render/graph/ms 7.49, stage/render/graph_submit 5.45, stage/render/specialize 3.60, stream/drive/ms 3.54 |
| warm-up | 6.30 | 35.1 | pipelines/compile_ms 78.55, pipelines/front_ms 74.61, render/thread/ms 44.22, render/busy/ms 43.12, main/busy/ms 34.66, stage/render/prepare_assets 22.84, encode//sum 18.28, stage/main/update 16.44, stage/main/post_update 11.28, stage/render/render 10.17, render/graph/ms 8.56, stage/render/graph_submit 6.26 |
| warm-up | 6.38 | 33.6 | render/thread/ms 42.20, render/busy/ms 40.93, main/busy/ms 35.68, stage/render/prepare_assets 19.92, encode//sum 18.96, stage/main/update 17.18, stage/main/post_update 11.95, stage/render/render 10.11, render/graph/ms 8.11, stage/render/graph_submit 5.82, stream/drive/ms 4.44, encode/finish_wall 3.43 |
| warm-up | 6.54 | 31.4 | render/thread/ms 52.38, render/busy/ms 51.11, main/busy/ms 42.84, encode//sum 29.18, stage/render/prepare_assets 26.94, stage/main/update 25.30, stream/drive/ms 18.85, stage/render/render 14.33, stage/main/post_update 13.09, render/graph/ms 12.45, stage/render/graph_submit 8.20, encode/finish_wall 4.61 |
| measure | 18.16 | 31.4 | main/busy/ms 39.30, stage/main/update 24.24, encode//sum 23.52, render/thread/ms 21.22, stream/drive/ms 20.74, render/busy/ms 20.12, stage/render/render 12.88, stage/main/post_update 12.43, render/graph/ms 11.37, stage/render/graph_submit 8.66, encode/submit 4.44, encode/finish_wall 4.31 |
| measure | 59.04 | 32.3 | render/thread/ms 42.69, render/busy/ms 40.70, main/busy/ms 35.96, encode//sum 30.35, stage/main/post_update 21.63, stage/render/render 21.04, render/graph/ms 18.53, stage/render/graph_submit 15.30, stage/render/prepare_assets 9.97, stage/main/update 9.49, encode/submit 8.47, encode/finish_wall 7.03 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_3casc\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 6%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **75%** (min 57%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.1 W · 71 °C (max 76) · VRAM 5544 MiB · throttled 100% of samples
