# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs,cascadeencoders,cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5219 frames over 65.7 s · **median 12.25 ms (82 fps)** · mean 12.58 · p95 15.12 · p99 17.18 (1% low 58 fps) · worst 62.82 ms · 28 spikes over 1.5× median · 18.3 stutters/min (work 21.0/min)
- warm-up: 323 frames over 9.7 s, worst 2695.7 ms
- hitches (> 30.6 ms): **3** measured, 43 in warm-up
- timed GPU passes: 7.32 ms of a 12.58 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.25 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- **FAIL** worst frame 62.82 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.072 | 2.075 | 2.203 | 1.00 | 0.174 |
| main_opaque_pass_3d | 1.664 | 1.667 | 2.064 | 1.00 | 0.053 |
| early prepass | 0.523 | 0.524 | 0.752 | 1.00 | 0.079 |
| bloom | 0.319 | 0.320 | 0.326 | 1.00 | 0.063 |
| atmosphere_luts | 0.314 | 0.314 | 0.371 | 1.00 | 0.003 |
| water_velocity | 0.282 | 0.283 | 0.353 | 1.00 | 0.001 |
| water_sspr | 0.205 | 0.205 | 0.205 | 1.00 | 0.002 |
| ssao | 0.193 | 0.193 | 0.195 | 1.00 | 0.007 |
| uniform_allocation | 0.182 | 0.182 | 0.186 | 1.00 | 0.030 |
| atmos_froxel_inject | 0.149 | 0.149 | 0.151 | 1.00 | 0.001 |
| clouds_trace | 0.142 | 0.142 | 0.234 | 1.00 | 0.001 |
| bin_unpacking | 0.134 | 0.134 | 0.143 | 1.00 | 0.022 |
| water_fft | 0.132 | 0.132 | 0.137 | 1.00 | 0.003 |
| upscaling | 0.119 | 0.119 | 0.127 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.002 |
| auxiliary_views | 0.098 | 0.098 | 0.495 | 1.00 | 0.090 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.084 | 0.084 | 0.162 | 1.00 | 0.008 |
| clustering | 0.064 | 0.064 | 0.068 | 1.00 | 0.019 |
| render_sky | 0.052 | 0.053 | 0.053 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.417 | 0.420 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.045 | 0.057 | 0.057 | 0.80 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.048 | 1.00 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.232 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.027 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.025 | 1.00 | 0.112 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 11.969 | 17.528 | 51.35 | 62468.2 |
| render/thread/ms | 11.311 | 13.710 | 61.18 | 59032.8 |
| render/busy/ms | 10.752 | 13.113 | 60.47 | 56115.8 |
| main/busy/ms | 9.445 | 12.127 | 29.10 | 49291.1 |
| stage/render/render | 7.487 | 9.230 | 57.73 | 39077.0 |
| render/graph/ms | 6.737 | 8.404 | 56.84 | 35162.4 |
| stage/main/post_update | 5.983 | 8.413 | 14.05 | 31225.4 |
| stage/render/graph_submit | 5.622 | 7.061 | 55.72 | 29342.6 |
| encode/finish_wall | 3.441 | 4.567 | 6.27 | 17959.2 |
| encode/submit | 2.223 | 2.901 | 52.70 | 11603.1 |
| stage/main/update | 2.176 | 2.622 | 22.74 | 11358.8 |
| stage/render/camera_driver | 1.058 | 1.397 | 4.59 | 5521.5 |
| stage/render/prepare_write_phase_buffers | 0.891 | 1.135 | 2.81 | 4649.1 |
| bigspace/total/ms | 0.682 | 2.486 | 5.25 | 3560.8 |
| stage/render/prepare_assets | 0.561 | 0.711 | 10.09 | 2928.4 |
| render/present/ms | 0.468 | 0.567 | 1.75 | 2440.7 |
| stage/main/first | 0.412 | 0.527 | 4.92 | 2149.2 |
| stage/render/prepare_meshes | 0.385 | 0.496 | 2.23 | 2010.4 |
| stage/render/prepare_resources | 0.384 | 0.479 | 1.00 | 2002.8 |
| stage/main/pre_update | 0.328 | 0.481 | 1.38 | 1711.8 |
| stage/render/specialize | 0.303 | 0.436 | 2.46 | 1579.5 |
| terrain/update/ms | 0.302 | 0.383 | 1.87 | 1576.1 |
| bigspace/low_precision/ms | 0.287 | 0.801 | 4.61 | 1498.9 |
| stage/render/queue | 0.269 | 0.431 | 3.12 | 1403.2 |
| controller/feed/ms | 0.269 | 0.344 | 0.76 | 1403.1 |
| terrain/select/ms | 0.268 | 0.340 | 0.69 | 1400.4 |
| stage/render/prepare_bind_groups | 0.252 | 0.420 | 2.72 | 1315.0 |
| stream/drive/ms | 0.249 | 0.334 | 20.66 | 1300.2 |
| stage/main/fixed_main_loop | 0.244 | 0.377 | 1.24 | 1271.6 |
| bigspace/high_precision/ms | 0.243 | 0.956 | 4.87 | 1270.3 |
| anim/evaluate/ms | 0.192 | 0.308 | 5.57 | 1003.5 |
| stream/tick/ms | 0.190 | 0.257 | 0.71 | 993.8 |
| far/select/ms | 0.184 | 0.246 | 0.52 | 961.3 |
| stage/render/prepare_flush | 0.168 | 0.337 | 0.89 | 875.5 |
| stage/render/create_views | 0.167 | 0.220 | 0.90 | 872.8 |
| stage/render/prepare_batch_phases | 0.165 | 0.234 | 0.49 | 859.8 |
| stage/render/extract_commands | 0.165 | 0.196 | 0.43 | 859.5 |
| controller/move/ms | 0.139 | 0.296 | 0.73 | 727.8 |
| anim/evaluate_cpu/ms | 0.139 | 0.174 | 0.45 | 724.9 |
| stage/render/prepare_views | 0.133 | 0.169 | 0.39 | 691.8 |
| stage/main/last | 0.121 | 0.191 | 1.65 | 632.3 |
| stage/render/phase_sort | 0.093 | 0.142 | 0.34 | 486.4 |
| render/acquire/ms | 0.091 | 0.120 | 0.36 | 476.3 |
| physics/step/ms | 0.054 | 0.087 | 0.52 | 283.2 |
| pipelines/compile_ms | 0.054 | 0.000 | 112.96 | 279.7 |
| pipelines/front_ms | 0.051 | 0.000 | 109.08 | 268.4 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.074 | 0.26 | 248.7 |
| physics/insert/ms | 0.037 | 0.385 | 0.89 | 191.4 |
| stage/render/cleanup | 0.033 | 0.108 | 0.62 | 172.2 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.34 | 159.4 |
| weather/map/ms | 0.026 | 0.034 | 0.21 | 135.1 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.47 | 76.8 |
| anim/apply/ms | 0.011 | 0.020 | 0.42 | 58.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.38 | 56.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.20 | 56.4 |
| controller/avoid/ms | 0.010 | 0.014 | 0.27 | 54.7 |
| script/update/ms | 0.007 | 0.009 | 0.22 | 35.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.08 | 28.1 |
| script/ui/ms | 0.005 | 0.006 | 0.12 | 26.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 25.2 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 23.9 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 17.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 14.0 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.88 | 11.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
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
| anim/evaluated/count | count | 19 | 30 | 20 |
| controller/movers/count | count | 20 | 21 | 20 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1301 | 2262 | 641 |
| crowd/c0/meshes/count | count | 354 | 646 | 174 |
| crowd/c1/count | count | 145 | 155 | 147 |
| crowd/c1/joints/count | count | 5539 | 5902 | 5555 |
| crowd/c1/meshes/count | count | 1165 | 1256 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4248 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.95 | 5.93 | 2.41 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.47 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.38 | 0.01 |
| encode/finish_critical | ms | 2.95 | 5.93 | 2.41 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.34 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.61 | 60.47 | 9.34 |
| gpu/timed/ms | ms | 7.98 | 11.66 | 8.67 |
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
| phases/alpha_mask3d/items | count | 827 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 827 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1287 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3444 | 5115 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1287 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3444 | 5115 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 220 | 27 |
| phases/shadow/bins | count | 4612 | 5678 | 2057 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.2 k | 12.8 k | 5350 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 195 | 251 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 298 | 299 | 299 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 599 | 11 |
| preprocess/bind_groups_created | count | 12 | 599 | 11 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 174 | 248 | 166 |
| process/cpu_usage | % | 25.24 | 41.03 | 25 |
| process/mem_usage | GiB | 5.39 | 5.55 | 5.55 |
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
| started_load_count |  | 4914 | 5009 | 5009 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.52 | 48.90 | 32.46 |
| system/mem_usage | % | 71.55 | 72.30 | 69.40 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 82 | 267 | 77 |
| views/all/bins | count | 7220 | 8611 | 5555 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 25.1 k | 13.0 k |
| views/all/pipelines | count | 240 | 293 | 73 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 17.0 k | 22.7 k | 13.3 k |
| views/cascade/batch_sets | count | 32 | 45 | 27 |
| views/cascade/bins | count | 2442 | 2993 | 2057 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7813 | 9726 | 5350 |
| views/cascade/pipelines | count | 32 | 45 | 27 |
| views/cascade/work_items | count | 7813 | 9726 | 5350 |
| views/lamp_face/batch_sets | count | 0 | 188 | 0 |
| views/lamp_face/bins | count | 2111 | 2764 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2368 | 3065 | 0 |
| views/lamp_face/pipelines | count | 162 | 210 | 0 |
| views/lamp_face/work_items | count | 0 | 2834 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2870 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8550 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8707 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 51.76 | 220 | 270.1 k |
| anim/events/count | count | 0.11 | 3 | 552 |
| assets/Image/added | count | 0.11 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.78 | 152 | 4061 |
| assets/Mesh/removed | count | 0.93 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 464 |
| assets/StandardMaterial/modified | count | 4.15 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.90 | 1492 | 4676 |
| encode//count | count | 36.88 | 272 | 192.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5219 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5219 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5219 |
| physics/inserts | count | 0.43 | 103 | 2252 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 145 | 2335 |
| physics/steps | count | 0.75 | 3 | 3938 |
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
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6486 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1422 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.6 ms; slowest 46, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.70 | 2695.7 | main/busy/ms 2666, stage/main/post_update 2564, stage/main/update 95.33, surface/advance_cpu_ms 66.04, stage/main/pre_update 2.57, stage/main/last 1.20, stage/main/fixed_main_loop 0.85, surface/cell_upload_ms 0.81, stream/drive/ms 0.48, stage/main/first 0.47, weather/map/ms 0.12, bigspace/total/ms 0.10 |
| warm-up | 3.48 | 786.4 | main/busy/ms 2805, stage/main/post_update 2588, render/thread/ms 754, render/busy/ms 602, stage/render/prepare_assets 554, stage/main/update 204, stage/render/prepare_views 150, render/acquire/ms 149, surface/advance_cpu_ms 66.04, stage/render/create_views 27.67, stage/render/render 13.04, encode//sum 10.82 |
| warm-up | 3.56 | 81.4 | render/thread/ms 819, render/busy/ms 665, stage/render/prepare_assets 600, main/busy/ms 167, stage/render/prepare_views 150, render/acquire/ms 150, stage/main/update 113, pipelines/compile_ms 41.87, stage/main/post_update 39.19, pipelines/front_ms 37.33, stage/render/create_views 28.01, stage/render/render 21.74 |
| warm-up | 3.61 | 43.8 | pipelines/compile_ms 228, pipelines/front_ms 206, pipelines/wait_ms 190, render/thread/ms 107, render/busy/ms 99.31, main/busy/ms 71.59, stage/main/post_update 49.38, stage/render/prepare_assets 48.18, stage/render/render 42.47, render/graph/ms 33.97, terrain/update/ms 27.79, pipelines/driver_ms 21.67 |
| warm-up | 3.66 | 49.4 | pipelines/compile_ms 579, pipelines/front_ms 537, pipelines/wait_ms 530, main/busy/ms 91.40, render/thread/ms 86.23, render/busy/ms 78.08, stage/main/post_update 50.07, pipelines/driver_ms 42.09, stage/render/render 37.94, render/graph/ms 29.51, terrain/update/ms 15.68, stage/render/camera_driver 13.05 |
| warm-up | 3.70 | 43.3 | pipelines/wait_ms 1378, pipelines/compile_ms 664, pipelines/front_ms 629, main/busy/ms 97.13, render/thread/ms 81.38, render/busy/ms 64.01, stage/main/post_update 35.29, pipelines/driver_ms 35.01, stage/main/update 18.84, terrain/update/ms 17.04, stage/main/fixed_main_loop 16.72, stage/render/prepare_assets 16.69 |
| warm-up | 3.79 | 86.6 | pipelines/wait_ms 3550, pipelines/compile_ms 948, pipelines/front_ms 901, main/busy/ms 136, render/thread/ms 110, render/busy/ms 93.36, stage/main/update 87.56, pipelines/driver_ms 47.52, stage/render/prepare_resources 33.88, stage/main/post_update 32.96, stage/render/prepare_assets 25.79, stage/render/prepare_bind_groups 18.95 |
| warm-up | 3.88 | 90.9 | pipelines/wait_ms 5839, pipelines/compile_ms 2434, pipelines/front_ms 2306, render/thread/ms 134, render/busy/ms 132, pipelines/driver_ms 128, main/busy/ms 113, stage/main/update 77.01, stage/render/prepare_assets 65.22, stage/render/prepare_resources 33.42, stage/main/post_update 24.33, stage/render/prepare_bind_groups 19.05 |
| warm-up | 3.91 | 33.0 | pipelines/wait_ms 3599, pipelines/compile_ms 2099, pipelines/front_ms 1997, pipelines/driver_ms 102, render/thread/ms 92.35, render/busy/ms 90.16, stage/render/prepare_assets 72.75, main/busy/ms 40.21, stage/main/post_update 18.39, stage/render/render 11.58, terrain/update/ms 10.79, stage/main/update 9.59 |
| warm-up | 4.08 | 69.7 | pipelines/compile_ms 943, pipelines/front_ms 918, render/thread/ms 58.68, render/busy/ms 56.90, main/busy/ms 52.51, stage/render/prepare_assets 40.50, stage/main/update 39.92, stream/drive/ms 35.92, pipelines/driver_ms 25.00, stage/render/render 11.01, pipelines/wait_ms 9.92, render/graph/ms 8.94 |
| warm-up | 4.19 | 77.7 | pipelines/compile_ms 468, pipelines/front_ms 451, render/thread/ms 69.84, render/busy/ms 67.57, main/busy/ms 67.20, stage/main/update 50.68, stream/drive/ms 45.38, stage/render/prepare_assets 37.03, stage/render/render 22.53, render/graph/ms 19.71, pipelines/driver_ms 17.11, pipelines/wait_ms 15.14 |
| warm-up | 4.31 | 99.3 | pipelines/compile_ms 447, pipelines/front_ms 432, render/thread/ms 78.72, render/busy/ms 73.91, main/busy/ms 58.14, stage/render/prepare_assets 38.82, stage/main/update 30.40, stage/render/render 21.48, stage/main/post_update 18.18, render/graph/ms 16.05, pipelines/driver_ms 14.41, stage/render/graph_submit 12.94 |
| warm-up | 4.38 | 69.4 | pipelines/compile_ms 1234, pipelines/front_ms 1196, render/thread/ms 109, render/busy/ms 104, stage/render/prepare_assets 72.04, main/busy/ms 63.35, pipelines/driver_ms 37.55, stage/main/update 37.04, stage/render/render 21.11, stage/main/post_update 15.65, render/graph/ms 15.60, stage/render/graph_submit 13.04 |
| warm-up | 4.48 | 103.5 | pipelines/compile_ms 1967, pipelines/front_ms 1910, render/thread/ms 124, render/busy/ms 122, encode//sum 93.59, stage/render/prepare_assets 72.94, pipelines/driver_ms 57.53, main/busy/ms 50.15, stage/render/render 41.46, render/graph/ms 39.25, stage/render/graph_submit 34.94, stage/main/update 32.48 |
| warm-up | 4.57 | 59.2 | pipelines/compile_ms 383, pipelines/front_ms 370, render/thread/ms 61.60, render/busy/ms 59.21, main/busy/ms 52.83, stage/render/render 25.69, stage/main/update 25.27, stage/render/prepare_assets 23.07, render/graph/ms 22.37, stage/main/post_update 17.33, stage/render/graph_submit 16.99, encode/submit 12.77 |
| warm-up | 4.64 | 71.5 | pipelines/compile_ms 889, pipelines/front_ms 857, pipelines/wait_ms 135, render/thread/ms 97.39, render/busy/ms 95.33, main/busy/ms 53.16, stage/render/prepare_assets 32.39, pipelines/driver_ms 32.36, stage/main/update 24.44, stage/main/post_update 22.94, encode//sum 21.56, stage/render/prepare_resources 20.91 |
| warm-up | 4.77 | 133.2 | pipelines/compile_ms 1865, pipelines/front_ms 1792, pipelines/wait_ms 545, render/thread/ms 187, render/busy/ms 184, encode//sum 171, main/busy/ms 135, stage/main/update 72.28, pipelines/driver_ms 72.23, stage/render/prepare_resources 60.06, stage/main/post_update 53.40, stage/render/render 37.01 |
| warm-up | 4.84 | 66.1 | pipelines/compile_ms 2837, pipelines/front_ms 2747, pipelines/wait_ms 506, encode//sum 168, render/thread/ms 167, render/busy/ms 163, main/busy/ms 162, stage/main/update 108, pipelines/driver_ms 89.20, stream/drive/ms 60.33, stage/render/prepare_assets 42.13, stage/main/post_update 40.69 |
| warm-up | 4.91 | 71.1 | pipelines/compile_ms 2428, pipelines/front_ms 2361, pipelines/wait_ms 556, render/thread/ms 81.03, main/busy/ms 80.49, render/busy/ms 79.32, pipelines/driver_ms 66.66, stage/main/update 59.27, stream/drive/ms 45.03, stage/render/prepare_assets 39.47, encode//sum 32.20, stage/render/render 21.47 |
| warm-up | 4.94 | 31.5 | pipelines/compile_ms 1010, pipelines/front_ms 977, pipelines/wait_ms 463, main/busy/ms 104, stage/main/update 84.56, render/thread/ms 67.88, render/busy/ms 65.81, stream/drive/ms 48.55, encode//sum 39.54, pipelines/driver_ms 32.99, stage/render/render 28.32, render/graph/ms 25.57 |
| warm-up | 5.08 | 134.0 | pipelines/compile_ms 1063, pipelines/front_ms 1038, main/busy/ms 130, stage/main/update 108, render/thread/ms 81.85, render/busy/ms 79.80, stream/drive/ms 67.43, stage/render/prepare_assets 34.98, encode//sum 31.23, stage/render/render 26.05, pipelines/driver_ms 25.03, render/graph/ms 23.48 |
| warm-up | 5.13 | 49.6 | pipelines/compile_ms 1435, pipelines/front_ms 1399, render/thread/ms 94.47, render/busy/ms 92.60, main/busy/ms 82.16, stage/main/update 57.77, stage/render/prepare_assets 40.93, stream/drive/ms 38.29, pipelines/driver_ms 36.22, stage/render/render 26.34, pipelines/wait_ms 24.74, render/graph/ms 23.84 |
| warm-up | 5.16 | 33.9 | pipelines/compile_ms 493, pipelines/front_ms 476, main/busy/ms 79.54, render/thread/ms 64.53, render/busy/ms 62.59, stage/main/update 60.78, stream/drive/ms 41.61, stage/render/render 24.21, stage/render/prepare_assets 21.88, render/graph/ms 21.67, encode//sum 18.63, pipelines/driver_ms 16.68 |
| warm-up | 5.21 | 47.5 | pipelines/compile_ms 355, pipelines/front_ms 343, main/busy/ms 76.53, stage/main/update 54.67, render/thread/ms 49.80, render/busy/ms 48.30, stream/drive/ms 33.85, stage/render/render 19.34, encode//sum 19.32, render/graph/ms 17.09, stage/render/prepare_assets 16.91, stage/main/post_update 15.12 |
| warm-up | 5.27 | 64.0 | pipelines/compile_ms 483, pipelines/front_ms 467, render/thread/ms 74.29, render/busy/ms 72.76, main/busy/ms 69.58, stage/main/update 34.09, stage/main/post_update 28.91, stage/render/render 21.63, encode//sum 21.36, render/graph/ms 19.39, stage/render/prepare_resources 17.29, pipelines/driver_ms 16.20 |
| warm-up | 5.34 | 63.2 | pipelines/compile_ms 1010, pipelines/front_ms 982, render/thread/ms 104, render/busy/ms 102, main/busy/ms 50.50, pipelines/driver_ms 28.17, stage/render/prepare_assets 26.12, stage/main/post_update 24.18, encode//sum 22.74, stage/main/update 18.78, stage/render/prepare_resources 18.51, stage/render/render 17.45 |
| warm-up | 5.37 | 35.3 | pipelines/compile_ms 1224, pipelines/front_ms 1188, render/thread/ms 88.36, render/busy/ms 86.65, main/busy/ms 44.25, pipelines/driver_ms 36.81, encode//sum 24.69, stage/render/prepare_assets 21.39, stage/main/update 20.38, stage/render/render 17.56, render/graph/ms 15.31, stage/main/post_update 15.06 |
| warm-up | 5.44 | 64.8 | pipelines/compile_ms 1778, pipelines/front_ms 1721, render/thread/ms 90.25, render/busy/ms 86.61, main/busy/ms 70.99, pipelines/driver_ms 57.18, stage/main/update 40.80, stage/render/render 22.10, encode//sum 21.20, stage/main/post_update 18.69, stage/render/prepare_resources 18.10, render/graph/ms 17.90 |
| warm-up | 5.48 | 45.3 | pipelines/compile_ms 1536, pipelines/front_ms 1487, render/thread/ms 88.78, render/busy/ms 85.22, main/busy/ms 64.23, pipelines/driver_ms 48.22, stage/main/update 37.43, stage/render/prepare_assets 21.31, stage/render/render 20.38, encode//sum 19.28, render/graph/ms 16.17, stage/main/post_update 15.04 |
| warm-up | 5.51 | 32.2 | pipelines/compile_ms 207, pipelines/front_ms 198, render/thread/ms 60.70, render/busy/ms 58.77, main/busy/ms 46.11, stage/render/render 23.44, encode//sum 21.66, render/graph/ms 20.63, stage/main/post_update 19.60, stage/main/update 18.72, stage/render/prepare_assets 17.56, stage/render/graph_submit 15.53 |
| warm-up | 5.58 | 34.5 | pipelines/compile_ms 350, pipelines/front_ms 339, render/thread/ms 52.44, render/busy/ms 50.72, main/busy/ms 41.03, encode//sum 20.48, stage/render/prepare_assets 20.12, stage/main/post_update 17.13, stage/main/update 16.67, stage/render/render 16.60, render/graph/ms 14.33, pipelines/driver_ms 11.13 |
| warm-up | 5.61 | 38.1 | pipelines/compile_ms 285, pipelines/front_ms 275, render/thread/ms 55.28, render/busy/ms 53.72, main/busy/ms 35.95, encode//sum 23.17, stage/render/prepare_assets 21.42, stage/render/render 15.78, stage/main/post_update 14.70, stage/main/update 14.60, render/graph/ms 13.56, pipelines/driver_ms 9.67 |
| warm-up | 5.65 | 36.2 | pipelines/compile_ms 266, pipelines/front_ms 257, render/thread/ms 61.56, render/busy/ms 59.97, main/busy/ms 38.85, encode//sum 21.20, stage/main/post_update 19.64, stage/render/render 18.52, render/graph/ms 15.99, stage/render/prepare_assets 13.89, stage/main/update 13.34, stage/render/prepare_resources 12.07 |
| warm-up | 5.69 | 35.9 | render/thread/ms 68.58, render/busy/ms 66.78, main/busy/ms 48.79, pipelines/compile_ms 45.90, pipelines/front_ms 43.14, stage/main/post_update 31.48, stage/render/prepare_resources 21.84, encode//sum 19.50, stage/render/render 19.07, render/graph/ms 16.57, stage/render/graph_submit 12.80, bigspace/total/ms 12.06 |
| warm-up | 5.73 | 40.5 | pipelines/compile_ms 1071, pipelines/front_ms 1042, render/thread/ms 71.24, render/busy/ms 69.26, main/busy/ms 40.05, pipelines/driver_ms 28.42, stage/render/prepare_assets 24.92, stage/main/post_update 23.86, encode//sum 19.58, stage/render/render 17.70, render/graph/ms 15.37, stage/render/prepare_resources 13.32 |
| warm-up | 5.76 | 36.3 | pipelines/compile_ms 1502, pipelines/front_ms 1461, render/thread/ms 70.07, render/busy/ms 66.34, main/busy/ms 41.75, pipelines/driver_ms 40.74, stage/render/prepare_assets 34.80, encode//sum 20.41, stage/main/update 18.57, stage/render/render 16.94, stage/main/post_update 14.61, render/graph/ms 14.53 |
| warm-up | 5.80 | 36.3 | pipelines/compile_ms 1301, pipelines/front_ms 1269, render/thread/ms 64.77, render/busy/ms 61.24, main/busy/ms 49.49, pipelines/driver_ms 31.98, stage/render/prepare_assets 31.21, encode//sum 25.17, stage/main/update 20.59, stage/main/post_update 20.30, stage/render/render 15.41, render/graph/ms 13.08 |
| warm-up | 5.87 | 41.8 | pipelines/compile_ms 219, pipelines/front_ms 215, render/thread/ms 60.26, render/busy/ms 58.64, main/busy/ms 54.24, stage/main/update 30.88, encode//sum 26.22, stage/render/render 22.48, render/graph/ms 20.10, stage/render/prepare_assets 18.74, stream/drive/ms 14.44, stage/render/graph_submit 14.05 |
| warm-up | 5.90 | 32.4 | pipelines/compile_ms 598, pipelines/front_ms 583, render/thread/ms 63.56, render/busy/ms 62.10, main/busy/ms 39.76, stage/render/prepare_assets 28.72, encode//sum 23.03, stage/main/update 19.30, stage/render/render 16.05, pipelines/driver_ms 15.03, render/graph/ms 13.68, stage/main/post_update 13.63 |
| warm-up | 5.94 | 36.4 | pipelines/compile_ms 976, pipelines/front_ms 952, render/thread/ms 60.38, render/busy/ms 59.01, stage/render/prepare_assets 30.80, main/busy/ms 25.69, pipelines/driver_ms 23.71, encode//sum 21.69, stage/render/render 15.22, render/graph/ms 13.11, stage/main/post_update 11.10, pipelines/wait_ms 10.93 |
| warm-up | 5.99 | 32.7 | pipelines/compile_ms 359, pipelines/front_ms 343, render/thread/ms 49.88, render/busy/ms 48.47, stage/render/prepare_assets 29.29, main/busy/ms 27.30, encode//sum 18.79, pipelines/driver_ms 16.62, stage/main/post_update 12.85, stage/render/render 12.51, render/graph/ms 10.24, stage/main/update 9.28 |
| warm-up | 6.55 | 34.5 | render/thread/ms 41.52, render/busy/ms 40.36, main/busy/ms 23.86, encode//sum 18.34, stage/render/prepare_assets 15.57, stage/render/render 10.58, stage/main/update 9.48, stage/main/post_update 9.37, render/graph/ms 8.98, stage/render/graph_submit 6.76, encode/finish_wall 4.51, stage/render/specialize 4.02 |
| warm-up | 7.28 | 33.4 | main/busy/ms 41.08, render/thread/ms 28.38, render/busy/ms 27.23, encode//sum 23.48, stage/main/update 22.80, stage/render/render 15.01, stream/drive/ms 13.70, render/graph/ms 13.33, stream/spawn/ms 12.46, stage/main/post_update 12.01, stage/render/graph_submit 10.31, encode/finish_wall 6.12 |
| measure | 3.53 | 62.8 | render/thread/ms 73.40, render/busy/ms 72.19, stage/render/render 65.30, render/graph/ms 63.71, stage/render/graph_submit 61.47, encode/submit 55.03, encode//sum 23.58, main/busy/ms 22.81, stage/main/post_update 15.33, encode/finish_wall 6.51, stage/main/update 5.02, stage/render/camera_driver 2.13 |
| measure | 18.18 | 32.2 | main/busy/ms 39.03, encode//sum 26.22, stage/main/update 24.59, render/thread/ms 24.12, render/busy/ms 22.92, stream/drive/ms 20.89, stage/render/render 14.47, render/graph/ms 12.83, stage/main/post_update 11.66, stage/render/graph_submit 10.49, encode/finish_wall 6.74, encode/submit 3.88 |
| measure | 59.05 | 33.2 | render/thread/ms 45.20, render/busy/ms 43.78, main/busy/ms 27.27, encode//sum 26.28, stage/render/render 16.35, stage/render/prepare_assets 16.08, stage/main/post_update 14.65, render/graph/ms 14.52, stage/render/graph_submit 11.37, encode/finish_wall 7.20, stage/main/update 6.69, encode/submit 4.24 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_off\balmora_walk@2560x1440_off=cascadeencoders_cascadehold_meshslabs-2 --bench-baseline none --off cascadeencoders,cascadehold,meshslabs
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 1%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **74%** (min 51%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.6 W · 71 °C (max 76) · VRAM 5950 MiB · throttled 100% of samples
