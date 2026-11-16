# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5442 frames over 65.7 s · **median 11.78 ms (85 fps)** · mean 12.06 · p95 14.67 · p99 17.26 (1% low 58 fps) · worst 57.86 ms · 43 spikes over 1.5× median · 30.2 stutters/min (work 28.3/min)
- warm-up: 324 frames over 9.5 s, worst 2661.1 ms
- hitches (> 29.5 ms): **3** measured, 40 in warm-up
- timed GPU passes: 7.85 ms of a 12.06 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.78 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 1.09 min = 30.16/min, limit 1/min
- **FAIL** worst frame 57.86 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.036 | 2.094 | 2.368 | 0.97 | 0.175 |
| main_opaque_pass_3d | 1.618 | 1.663 | 2.166 | 0.97 | 0.055 |
| per_view_shadow_pass_cascade1 | 0.586 | 0.602 | 0.904 | 0.97 | 0.043 |
| early prepass | 0.522 | 0.537 | 0.774 | 0.97 | 0.079 |
| bloom | 0.314 | 0.323 | 0.327 | 0.97 | 0.065 |
| atmosphere_luts | 0.310 | 0.319 | 0.385 | 0.97 | 0.003 |
| water_velocity | 0.280 | 0.288 | 0.354 | 0.97 | 0.002 |
| auxiliary_views | 0.202 | 0.208 | 0.695 | 0.97 | 0.183 |
| water_sspr | 0.202 | 0.208 | 0.206 | 0.97 | 0.002 |
| ssao | 0.191 | 0.197 | 0.196 | 0.97 | 0.007 |
| uniform_allocation | 0.174 | 0.179 | 0.469 | 0.97 | 0.029 |
| atmos_froxel_inject | 0.146 | 0.150 | 0.150 | 0.97 | 0.001 |
| clouds_trace | 0.137 | 0.141 | 0.232 | 0.97 | 0.001 |
| water_fft | 0.131 | 0.134 | 0.138 | 0.97 | 0.003 |
| bin_unpacking | 0.117 | 0.121 | 0.129 | 0.97 | 0.022 |
| upscaling | 0.117 | 0.121 | 0.129 | 0.97 | 0.004 |
| clouds_composite | 0.106 | 0.109 | 0.110 | 0.97 | 0.002 |
| tonemapping | 0.085 | 0.087 | 0.090 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.082 | 0.085 | 0.161 | 0.97 | 0.008 |
| clustering | 0.064 | 0.065 | 0.069 | 0.97 | 0.019 |
| render_sky | 0.051 | 0.053 | 0.053 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.440 | 0.728 | 0.11 | 0.001 |
| surface_trail | 0.043 | 0.058 | 0.057 | 0.75 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.043 | 0.045 | 0.97 | 0.005 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.97 | 0.001 |
| water_snapshot | 0.025 | 0.025 | 0.028 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.224 | 0.234 | 0.11 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.015 | 0.016 | 0.97 | 0.005 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.025 | 0.97 | 0.114 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.017 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.044 | 0.048 | 0.01 | 0.015 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.018 | 0.020 | 0.01 | 0.010 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 12.545 | 18.420 | 143.55 | 68268.4 |
| render/thread/ms | 10.771 | 13.263 | 56.87 | 58616.9 |
| render/busy/ms | 10.178 | 12.678 | 56.35 | 55386.5 |
| main/busy/ms | 9.622 | 11.835 | 28.86 | 52360.9 |
| stage/render/render | 6.914 | 8.755 | 52.44 | 37627.5 |
| render/graph/ms | 6.130 | 7.933 | 51.74 | 33361.8 |
| stage/main/post_update | 6.115 | 8.041 | 16.46 | 33279.4 |
| stage/render/graph_submit | 4.918 | 6.388 | 50.13 | 26765.7 |
| encode/submit | 2.513 | 3.398 | 6.42 | 13675.5 |
| encode/finish_wall | 2.478 | 3.300 | 47.14 | 13485.8 |
| stage/main/update | 2.216 | 2.691 | 22.60 | 12060.9 |
| stage/render/camera_driver | 1.155 | 1.582 | 4.05 | 6288.1 |
| stage/render/prepare_write_phase_buffers | 0.875 | 1.144 | 3.67 | 4764.4 |
| bigspace/total/ms | 0.649 | 2.191 | 4.02 | 3529.8 |
| stage/render/prepare_assets | 0.565 | 0.725 | 9.10 | 3072.2 |
| render/present/ms | 0.500 | 0.626 | 1.93 | 2718.6 |
| stage/render/prepare_resources | 0.457 | 0.565 | 1.43 | 2487.9 |
| stage/main/first | 0.404 | 0.533 | 5.20 | 2197.1 |
| stage/render/prepare_meshes | 0.390 | 0.512 | 1.86 | 2120.5 |
| stage/main/pre_update | 0.339 | 0.516 | 1.15 | 1844.9 |
| terrain/update/ms | 0.305 | 0.394 | 1.99 | 1662.2 |
| stage/render/specialize | 0.283 | 0.422 | 3.12 | 1537.7 |
| bigspace/low_precision/ms | 0.272 | 1.145 | 3.48 | 1480.9 |
| terrain/select/ms | 0.272 | 0.353 | 0.80 | 1479.9 |
| controller/feed/ms | 0.270 | 0.353 | 0.66 | 1467.3 |
| stream/drive/ms | 0.252 | 0.350 | 20.46 | 1370.0 |
| stage/render/prepare_bind_groups | 0.252 | 0.427 | 2.96 | 1368.7 |
| stage/main/fixed_main_loop | 0.240 | 0.392 | 1.20 | 1304.6 |
| bigspace/high_precision/ms | 0.224 | 1.209 | 3.23 | 1217.3 |
| stage/render/queue | 0.215 | 0.371 | 2.53 | 1172.0 |
| anim/evaluate/ms | 0.194 | 0.302 | 2.65 | 1058.2 |
| stream/tick/ms | 0.192 | 0.263 | 0.63 | 1047.0 |
| far/select/ms | 0.191 | 0.264 | 0.59 | 1041.8 |
| stage/render/prepare_flush | 0.184 | 0.373 | 1.01 | 999.6 |
| stage/render/create_views | 0.182 | 0.244 | 1.05 | 992.3 |
| stage/render/extract_commands | 0.178 | 0.218 | 0.48 | 969.9 |
| stage/render/prepare_batch_phases | 0.169 | 0.240 | 0.60 | 922.3 |
| anim/evaluate_cpu/ms | 0.146 | 0.186 | 0.45 | 794.8 |
| stage/render/prepare_views | 0.136 | 0.176 | 0.44 | 741.4 |
| controller/move/ms | 0.135 | 0.284 | 0.59 | 736.2 |
| stage/main/last | 0.125 | 0.207 | 1.71 | 680.4 |
| render/acquire/ms | 0.094 | 0.124 | 0.31 | 511.8 |
| stage/render/phase_sort | 0.080 | 0.110 | 0.44 | 434.8 |
| physics/step/ms | 0.053 | 0.088 | 0.61 | 288.6 |
| pipelines/compile_ms | 0.052 | 0.000 | 112.55 | 283.6 |
| pipelines/front_ms | 0.050 | 0.000 | 108.74 | 272.1 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.077 | 0.33 | 262.1 |
| physics/insert/ms | 0.036 | 0.385 | 0.83 | 196.7 |
| stage/render/cleanup | 0.033 | 0.096 | 0.61 | 179.3 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.33 | 166.3 |
| weather/map/ms | 0.026 | 0.034 | 0.28 | 142.8 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.15 | 80.2 |
| anim/apply/ms | 0.012 | 0.023 | 0.27 | 65.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.34 | 59.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.14 | 59.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.20 | 57.2 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 36.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.03 | 29.8 |
| script/ui/ms | 0.005 | 0.007 | 0.16 | 28.5 |
| script/input/ms | 0.005 | 0.006 | 0.25 | 26.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.1 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 19.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.61 | 14.1 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.81 | 11.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.8 |
| stage/render/prepare | 0.000 | 0.003 | 0.03 | 2.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 3 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1247 | 2262 | 583 |
| crowd/c0/meshes/count | count | 349 | 646 | 162 |
| crowd/c1/count | count | 147 | 156 | 147 |
| crowd/c1/joints/count | count | 5581 | 5982 | 5581 |
| crowd/c1/meshes/count | count | 1181 | 1284 | 1171 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1822 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 188 | 236 | 223 |
| crowd/joints_extracted/count | count | 4277 | 9879 | 6179 |
| crowd/joints_uploaded/count | count | 4262 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.97 | 47.05 | 1.84 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.15 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.34 | 0.01 |
| encode/finish_critical | ms | 1.97 | 47.05 | 1.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.33 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.23 | 56.35 | 10.13 |
| gpu/timed/ms | ms | 8.80 | 11.99 | 9.34 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1297 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3378 | 5156 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1297 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3378 | 5156 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 215 | 19 |
| phases/shadow/bins | count | 4230 | 5438 | 1793 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9458 | 12.1 k | 4889 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 189 | 244 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 200 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 10 | 588 | 11 |
| preprocess/bind_groups_created | count | 10 | 588 | 11 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 163 | 253 | 149 |
| process/cpu_usage | % | 26.92 | 38.46 | 25 |
| process/mem_usage | GiB | 5.32 | 5.49 | 5.49 |
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
| started_load_count |  | 4919 | 5013 | 5013 |
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
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.38 | 63.34 | 32.46 |
| system/mem_usage | % | 67.71 | 68.42 | 68.29 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 78 | 262 | 69 |
| views/all/bins | count | 6820 | 8371 | 5291 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.5 k | 24.7 k | 12.5 k |
| views/all/pipelines | count | 234 | 291 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 16.3 k | 22.3 k | 12.8 k |
| views/cascade/batch_sets | count | 24 | 33 | 19 |
| views/cascade/bins | count | 2084 | 2694 | 1793 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6972 | 9025 | 4889 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 6972 | 9025 | 4889 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2131 | 2921 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2383 | 3242 | 0 |
| views/lamp_face/pipelines | count | 164 | 213 | 0 |
| views/lamp_face/work_items | count | 0 | 2875 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2892 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8366 | 13.1 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8534 | 13.3 k | 7926 |
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
| anim/bones_written/count | count | 62.89 | 287 | 342.2 k |
| anim/events/count | count | 0.11 | 3 | 587 |
| assets/Image/added | count | 0.11 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.75 | 152 | 4061 |
| assets/Mesh/removed | count | 0.89 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.98 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.86 | 1455 | 4676 |
| encode//count | count | 42.59 | 273 | 231.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5442 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5442 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5442 |
| physics/inserts | count | 0.41 | 109 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 118 | 2335 |
| physics/steps | count | 0.72 | 3 | 3938 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
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
| stream/spawn/items | count | 0.07 | 18 | 378 |
| stream/upload/bytes | bytes | 6221 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1364 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.5 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2661.1 | main/busy/ms 2631, stage/main/post_update 2527, stage/main/update 96.73, surface/advance_cpu_ms 66.29, stage/main/pre_update 2.55, stage/main/last 1.46, surface/cell_upload_ms 0.81, stage/main/first 0.54, stream/drive/ms 0.41, stage/main/fixed_main_loop 0.35, weather/map/ms 0.10, bigspace/total/ms 0.09 |
| warm-up | 3.48 | 823.0 | main/busy/ms 2774, stage/main/post_update 2550, render/thread/ms 789, render/busy/ms 620, stage/render/prepare_assets 582, stage/main/update 211, stage/render/prepare_views 164, render/acquire/ms 163, surface/advance_cpu_ms 66.29, stage/render/prepare_resources 21.96, stage/render/render 15.13, encode//sum 9.36 |
| warm-up | 3.57 | 89.3 | render/thread/ms 861, render/busy/ms 691, stage/render/prepare_assets 631, main/busy/ms 173, stage/render/prepare_views 165, render/acquire/ms 163, stage/main/update 121, stage/main/post_update 36.25, stage/render/render 28.55, stage/render/prepare_resources 27.43, render/graph/ms 19.91, pipelines/compile_ms 19.67 |
| warm-up | 3.64 | 39.0 | pipelines/wait_ms 597, pipelines/compile_ms 488, pipelines/front_ms 441, main/busy/ms 56.79, pipelines/driver_ms 47.29, render/thread/ms 45.24, stage/main/post_update 39.25, render/busy/ms 35.34, terrain/update/ms 20.39, stage/render/render 15.58, render/graph/ms 9.48, stage/main/update 8.82 |
| warm-up | 3.76 | 120.0 | pipelines/wait_ms 4908, pipelines/compile_ms 1409, pipelines/front_ms 1327, render/thread/ms 145, render/busy/ms 138, pipelines/driver_ms 81.77, main/busy/ms 71.11, stage/render/prepare_resources 34.13, stage/main/update 31.61, stage/render/prepare_write_phase_buffers 29.10, stage/render/prepare_assets 25.98, stage/main/post_update 25.38 |
| warm-up | 3.81 | 47.8 | pipelines/wait_ms 4593, pipelines/compile_ms 1477, pipelines/front_ms 1400, render/thread/ms 165, render/busy/ms 162, main/busy/ms 127, stage/main/update 78.80, pipelines/driver_ms 76.89, stage/render/prepare_resources 41.50, stage/render/prepare_assets 39.21, stage/render/prepare_write_phase_buffers 32.65, stage/render/render 29.58 |
| warm-up | 3.96 | 154.2 | pipelines/compile_ms 1582, pipelines/front_ms 1485, pipelines/wait_ms 556, render/thread/ms 164, render/busy/ms 121, main/busy/ms 100, pipelines/driver_ms 97.42, stage/render/prepare_assets 80.12, stage/render/render 62.25, stage/main/update 55.48, render/present/ms 42.57, stage/main/post_update 20.70 |
| warm-up | 4.03 | 43.8 | pipelines/compile_ms 1654, pipelines/front_ms 1483, pipelines/wait_ms 236, pipelines/driver_ms 171, render/thread/ms 62.92, render/busy/ms 60.53, main/busy/ms 46.34, stage/render/prepare_assets 38.54, stage/main/update 20.75, stage/main/post_update 17.18, stage/render/render 15.06, terrain/update/ms 12.84 |
| warm-up | 4.15 | 55.7 | pipelines/compile_ms 922, pipelines/front_ms 883, main/busy/ms 51.17, render/thread/ms 48.72, render/busy/ms 47.07, pipelines/driver_ms 38.52, stage/main/update 36.51, stream/drive/ms 32.14, stage/render/prepare_assets 29.35, pipelines/wait_ms 14.38, stage/render/render 12.31, stage/main/post_update 10.74 |
| warm-up | 4.19 | 38.4 | pipelines/compile_ms 598, pipelines/front_ms 577, render/thread/ms 65.52, render/busy/ms 63.85, main/busy/ms 55.11, stage/render/prepare_assets 42.18, stage/main/update 41.31, stream/drive/ms 33.28, pipelines/driver_ms 21.26, stage/render/render 16.65, render/graph/ms 14.51, pipelines/wait_ms 10.33 |
| warm-up | 4.24 | 47.3 | pipelines/compile_ms 528, pipelines/front_ms 504, render/thread/ms 70.05, render/busy/ms 67.24, main/busy/ms 62.68, stage/main/update 43.74, stage/render/prepare_assets 40.31, stream/drive/ms 34.48, pipelines/driver_ms 24.11, stage/render/render 19.05, render/graph/ms 15.75, stage/main/post_update 10.73 |
| warm-up | 4.27 | 34.7 | pipelines/compile_ms 677, pipelines/front_ms 648, render/thread/ms 62.27, render/busy/ms 59.56, main/busy/ms 51.18, stage/render/prepare_assets 33.98, stage/main/update 29.66, pipelines/driver_ms 29.44, stream/drive/ms 21.39, stage/main/post_update 14.24, stage/render/render 12.14, render/graph/ms 9.14 |
| warm-up | 4.35 | 73.4 | pipelines/compile_ms 292, pipelines/front_ms 281, render/thread/ms 79.53, render/busy/ms 77.29, main/busy/ms 62.24, stage/render/prepare_assets 47.69, stage/main/update 29.65, stage/main/post_update 19.70, stage/render/render 15.65, stream/drive/ms 13.52, render/graph/ms 12.85, pipelines/driver_ms 11.90 |
| warm-up | 4.43 | 81.5 | pipelines/compile_ms 1290, pipelines/front_ms 1256, render/thread/ms 112, render/busy/ms 110, stage/render/prepare_assets 85.20, main/busy/ms 70.18, stage/main/update 38.75, pipelines/driver_ms 34.13, stage/main/post_update 16.84, stage/render/render 16.57, render/graph/ms 13.76, stage/render/graph_submit 9.89 |
| warm-up | 4.56 | 129.6 | pipelines/compile_ms 1731, pipelines/front_ms 1685, render/thread/ms 151, render/busy/ms 149, stage/render/prepare_assets 92.91, main/busy/ms 56.14, pipelines/driver_ms 45.59, stage/main/update 36.83, stage/render/prepare_resources 30.64, stage/render/render 19.42, render/graph/ms 17.06, stage/main/post_update 9.69 |
| warm-up | 4.61 | 54.4 | pipelines/compile_ms 1289, pipelines/front_ms 1243, render/thread/ms 138, render/busy/ms 134, main/busy/ms 89.32, stage/render/prepare_assets 50.71, pipelines/driver_ms 46.10, stage/main/post_update 43.57, stage/main/update 35.14, stage/render/prepare_resources 31.33, stage/render/render 31.27, render/graph/ms 28.44 |
| warm-up | 4.73 | 115.8 | pipelines/compile_ms 952, pipelines/front_ms 915, main/busy/ms 122, render/thread/ms 120, render/busy/ms 115, stage/main/post_update 75.03, stage/render/prepare_assets 63.11, pipelines/driver_ms 36.65, stage/main/update 30.72, bigspace/total/ms 25.15, bigspace/high_precision/ms 24.52, stage/render/render 24.30 |
| warm-up | 4.84 | 106.9 | pipelines/compile_ms 2705, pipelines/front_ms 2616, pipelines/wait_ms 481, render/thread/ms 159, render/busy/ms 157, stage/render/prepare_assets 116, main/busy/ms 99.89, pipelines/driver_ms 88.96, stage/main/post_update 58.34, bigspace/total/ms 24.96, bigspace/high_precision/ms 24.51, stage/main/update 23.43 |
| warm-up | 4.90 | 67.8 | pipelines/compile_ms 3098, pipelines/front_ms 3000, pipelines/wait_ms 625, render/thread/ms 142, render/busy/ms 140, pipelines/driver_ms 98.02, main/busy/ms 87.51, stage/render/prepare_assets 75.23, stage/main/post_update 41.45, stage/main/update 34.02, encode//sum 30.18, stage/render/render 24.15 |
| warm-up | 4.96 | 60.1 | pipelines/compile_ms 1072, pipelines/front_ms 1036, pipelines/wait_ms 276, render/thread/ms 109, main/busy/ms 109, render/busy/ms 107, encode//sum 84.16, stage/main/update 65.56, stream/drive/ms 55.36, pipelines/driver_ms 36.14, stage/render/render 30.11, render/graph/ms 27.16 |
| warm-up | 5.02 | 60.3 | pipelines/compile_ms 1894, pipelines/front_ms 1831, pipelines/wait_ms 548, render/thread/ms 92.07, render/busy/ms 89.67, main/busy/ms 87.93, encode//sum 81.14, pipelines/driver_ms 63.52, stage/main/update 54.91, stream/drive/ms 42.81, stage/render/render 34.16, render/graph/ms 30.73 |
| warm-up | 5.16 | 122.9 | pipelines/compile_ms 1007, pipelines/front_ms 987, main/busy/ms 107, stage/main/update 88.16, render/thread/ms 56.95, render/busy/ms 55.46, stream/drive/ms 55.22, pipelines/wait_ms 38.49, stage/render/prepare_assets 28.03, pipelines/driver_ms 20.52, encode//sum 14.37, stage/render/render 13.14 |
| warm-up | 5.20 | 40.3 | pipelines/compile_ms 835, pipelines/front_ms 809, render/thread/ms 79.59, main/busy/ms 76.26, render/busy/ms 75.86, stage/main/update 56.63, stream/drive/ms 40.51, pipelines/wait_ms 32.90, stage/render/prepare_assets 31.11, pipelines/driver_ms 25.57, stage/render/render 21.26, encode//sum 18.11 |
| warm-up | 5.25 | 50.5 | pipelines/compile_ms 295, pipelines/front_ms 279, main/busy/ms 72.69, render/thread/ms 66.25, render/busy/ms 62.44, stage/main/update 53.38, stream/drive/ms 33.01, stage/render/prepare_assets 19.37, stage/render/render 18.93, encode//sum 18.66, pipelines/driver_ms 16.15, render/graph/ms 15.01 |
| warm-up | 5.33 | 48.3 | pipelines/compile_ms 285, pipelines/front_ms 276, main/busy/ms 56.53, render/thread/ms 46.42, render/busy/ms 44.63, stage/main/update 41.41, stream/drive/ms 27.69, encode//sum 17.96, stage/render/render 15.00, stage/render/prepare_assets 12.78, render/graph/ms 12.51, stage/render/graph_submit 9.56 |
| warm-up | 5.39 | 31.2 | pipelines/compile_ms 1154, pipelines/front_ms 1118, render/thread/ms 49.34, render/busy/ms 47.10, pipelines/driver_ms 35.67, main/busy/ms 33.78, encode//sum 18.48, stage/main/update 15.51, stage/render/render 15.01, stage/render/prepare_assets 14.29, stage/main/post_update 12.75, render/graph/ms 12.68 |
| warm-up | 5.51 | 43.2 | pipelines/compile_ms 653, pipelines/front_ms 632, render/thread/ms 51.50, render/busy/ms 49.62, main/busy/ms 43.38, stage/main/update 26.69, stage/render/prepare_assets 24.96, encode//sum 23.63, pipelines/driver_ms 21.01, stage/render/render 17.41, render/graph/ms 14.77, stream/drive/ms 12.55 |
| warm-up | 5.57 | 31.1 | pipelines/compile_ms 307, pipelines/front_ms 295, render/thread/ms 52.90, render/busy/ms 50.23, main/busy/ms 33.75, stage/render/render 22.31, render/graph/ms 19.73, encode//sum 17.05, stage/main/post_update 15.53, stage/render/graph_submit 13.25, stage/main/update 12.11, pipelines/driver_ms 11.54 |
| warm-up | 5.60 | 34.7 | pipelines/compile_ms 278, pipelines/front_ms 267, render/thread/ms 60.38, render/busy/ms 58.28, main/busy/ms 38.78, stage/render/render 22.04, render/graph/ms 19.47, encode//sum 17.64, stage/main/post_update 15.80, stage/main/update 15.67, stage/render/graph_submit 13.31, stage/render/prepare_assets 11.46 |
| warm-up | 5.67 | 36.5 | pipelines/compile_ms 59.32, pipelines/front_ms 56.11, render/thread/ms 55.49, render/busy/ms 53.99, main/busy/ms 44.69, stage/render/prepare_assets 24.90, stage/main/update 20.18, encode//sum 19.21, stage/main/post_update 18.39, stream/drive/ms 13.35, stage/render/render 12.33, render/graph/ms 10.10 |
| warm-up | 5.71 | 45.0 | pipelines/compile_ms 1046, pipelines/front_ms 1015, render/thread/ms 74.11, render/busy/ms 72.54, main/busy/ms 55.75, stage/render/prepare_assets 36.30, pipelines/driver_ms 31.19, stage/main/update 29.43, stage/main/post_update 19.83, encode//sum 18.56, stream/drive/ms 13.51, stage/render/render 12.70 |
| warm-up | 5.76 | 44.7 | pipelines/compile_ms 1884, pipelines/front_ms 1830, render/thread/ms 81.70, render/busy/ms 79.97, pipelines/driver_ms 53.08, main/busy/ms 48.00, stage/render/prepare_assets 45.58, stage/main/post_update 21.33, stage/main/update 20.21, encode//sum 18.14, stage/render/render 13.19, render/graph/ms 10.81 |
| warm-up | 5.85 | 62.4 | pipelines/compile_ms 648, pipelines/front_ms 631, render/thread/ms 83.12, render/busy/ms 80.90, main/busy/ms 50.87, stage/render/prepare_assets 27.70, stage/main/post_update 20.32, encode//sum 17.13, pipelines/driver_ms 16.68, stage/render/render 15.16, stage/main/update 14.11, render/graph/ms 12.99 |
| warm-up | 5.88 | 34.0 | pipelines/compile_ms 783, pipelines/front_ms 755, render/thread/ms 86.22, render/busy/ms 83.04, main/busy/ms 57.49, stage/render/prepare_assets 38.71, pipelines/driver_ms 28.56, stage/main/post_update 25.24, encode//sum 18.18, stage/main/update 15.26, stage/render/render 14.66, stage/main/fixed_main_loop 13.15 |
| warm-up | 6.40 | 31.0 | render/thread/ms 42.96, render/busy/ms 41.68, main/busy/ms 27.98, encode//sum 17.66, stage/render/prepare_assets 17.15, stage/main/update 11.99, stage/render/render 10.87, stage/main/post_update 10.07, render/graph/ms 9.22, stage/render/graph_submit 7.15, stage/render/specialize 4.34, encode/submit 3.76 |
| warm-up | 6.60 | 33.3 | main/busy/ms 46.42, render/thread/ms 43.05, render/busy/ms 41.38, encode//sum 27.84, stage/main/update 22.73, stage/main/post_update 16.33, stage/render/render 16.01, render/graph/ms 13.48, stage/render/graph_submit 9.68, stage/render/prepare_assets 7.88, stream/drive/ms 5.79, encode/finish_wall 5.38 |
| warm-up | 6.65 | 30.0 | pipelines/compile_ms 81.81, pipelines/front_ms 76.20, render/thread/ms 39.94, render/busy/ms 38.70, main/busy/ms 30.39, stage/render/prepare_assets 19.15, encode//sum 17.88, stage/main/update 14.19, stage/main/post_update 9.97, stage/render/render 9.89, render/graph/ms 8.09, stage/render/graph_submit 6.06 |
| warm-up | 6.68 | 30.2 | pipelines/compile_ms 81.81, pipelines/front_ms 76.20, render/thread/ms 49.61, render/busy/ms 48.10, main/busy/ms 38.01, encode//sum 28.18, stage/render/render 18.20, stage/main/post_update 16.30, render/graph/ms 16.05, stage/main/update 15.54, stage/render/prepare_assets 15.09, stage/render/graph_submit 12.16 |
| warm-up | 6.73 | 30.2 | render/thread/ms 39.93, render/busy/ms 38.75, main/busy/ms 36.52, encode//sum 23.26, stage/main/update 16.50, stage/main/post_update 14.14, stage/render/render 13.43, stage/render/prepare_assets 12.89, render/graph/ms 11.80, stage/render/graph_submit 8.81, encode/finish_wall 5.12, encode/submit 3.74 |
| warm-up | 6.89 | 31.2 | render/thread/ms 41.43, render/busy/ms 39.95, main/busy/ms 36.76, stage/main/update 22.50, encode//sum 20.11, stage/render/prepare_assets 18.93, stream/drive/ms 16.62, stage/render/render 13.21, render/graph/ms 11.17, stage/main/post_update 10.38, stage/render/graph_submit 9.02, encode/submit 4.59 |
| measure | 1.10 | 43.8 | encode//sum 154, render/thread/ms 51.30, render/busy/ms 50.18, stage/render/render 44.52, render/graph/ms 42.96, stage/render/graph_submit 40.70, encode/finish_wall 36.79, main/busy/ms 19.86, stage/main/post_update 13.01, stage/main/update 4.37, encode/submit 4.00, stage/render/camera_driver 2.16 |
| measure | 1.72 | 57.9 | encode//sum 115, render/thread/ms 66.10, render/busy/ms 65.03, stage/render/render 58.29, render/graph/ms 56.91, stage/render/graph_submit 54.17, encode/finish_wall 49.03, main/busy/ms 18.92, stage/main/post_update 11.75, encode/submit 5.19, stage/main/update 4.77, stage/render/camera_driver 2.61 |
| measure | 59.03 | 29.5 | render/thread/ms 38.61, render/busy/ms 36.49, encode//sum 34.15, main/busy/ms 33.55, stage/main/post_update 20.16, stage/render/render 19.46, render/graph/ms 17.09, stage/render/graph_submit 13.17, stage/main/update 8.50, encode/submit 7.02, stage/render/prepare_assets 6.64, encode/finish_wall 6.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_nohold\balmora_walk@2560x1440_off=cascadehold-1 --bench-baseline none --off cascadehold
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **80%** (min 47%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.3 W · 72 °C (max 78) · VRAM 5972 MiB · throttled 100% of samples
