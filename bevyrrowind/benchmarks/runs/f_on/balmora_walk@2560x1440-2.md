# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5631 frames over 65.6 s · **median 11.34 ms (88 fps)** · mean 11.66 · p95 14.29 · p99 16.76 (1% low 60 fps) · worst 30.76 ms · 45 spikes over 1.5× median · 27.4 stutters/min (work 21.9/min)
- warm-up: 325 frames over 9.3 s, worst 2727.3 ms
- hitches (> 28.4 ms): **3** measured, 39 in warm-up
- timed GPU passes: 7.31 ms of a 11.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.34 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 30.76 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.040 | 2.081 | 2.254 | 0.98 | 0.177 |
| main_opaque_pass_3d | 1.602 | 1.634 | 2.043 | 0.98 | 0.067 |
| early prepass | 0.546 | 0.557 | 1.114 | 0.98 | 0.080 |
| bloom | 0.315 | 0.321 | 0.326 | 0.98 | 0.065 |
| atmosphere_luts | 0.312 | 0.319 | 0.422 | 0.98 | 0.003 |
| water_velocity | 0.279 | 0.285 | 0.353 | 0.98 | 0.002 |
| water_sspr | 0.202 | 0.206 | 0.206 | 0.98 | 0.002 |
| ssao | 0.190 | 0.193 | 0.196 | 0.98 | 0.007 |
| uniform_allocation | 0.153 | 0.156 | 0.439 | 0.98 | 0.025 |
| auxiliary_views | 0.147 | 0.150 | 0.592 | 0.98 | 0.139 |
| atmos_froxel_inject | 0.145 | 0.148 | 0.148 | 0.98 | 0.001 |
| per_view_shadow_pass_cascade1 | 0.143 | 0.575 | 0.651 | 0.25 | 0.043 |
| clouds_trace | 0.139 | 0.142 | 0.236 | 0.98 | 0.001 |
| water_fft | 0.135 | 0.138 | 0.143 | 0.98 | 0.003 |
| upscaling | 0.116 | 0.118 | 0.128 | 0.98 | 0.004 |
| clouds_composite | 0.106 | 0.108 | 0.110 | 0.98 | 0.002 |
| bin_unpacking | 0.099 | 0.101 | 0.123 | 0.98 | 0.018 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.083 | 0.085 | 0.162 | 0.98 | 0.008 |
| clustering | 0.064 | 0.065 | 0.068 | 0.98 | 0.019 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.048 | 0.423 | 0.421 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.98 | 0.001 |
| surface_trail | 0.043 | 0.058 | 0.057 | 0.74 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.031 | 0.043 | 0.98 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.230 | 0.237 | 0.11 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.025 | 0.98 | 0.115 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.98 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.013 | 0.11 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.040 | 0.051 | 0.01 | 0.013 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.045 | 0.050 | 0.00 | 0.017 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.558 | 16.308 | 48.54 | 59452.4 |
| render/thread/ms | 10.337 | 12.804 | 26.72 | 58209.4 |
| render/busy/ms | 9.752 | 12.202 | 26.07 | 54912.9 |
| main/busy/ms | 9.408 | 11.691 | 29.12 | 52977.0 |
| stage/render/render | 6.541 | 8.304 | 16.48 | 36833.2 |
| stage/main/post_update | 5.907 | 7.797 | 12.03 | 33262.6 |
| render/graph/ms | 5.767 | 7.455 | 15.22 | 32473.0 |
| stage/render/graph_submit | 4.587 | 5.921 | 10.91 | 25829.4 |
| encode/submit | 2.391 | 3.286 | 6.59 | 13466.2 |
| encode/finish_wall | 2.260 | 2.953 | 5.22 | 12723.7 |
| stage/main/update | 2.225 | 2.712 | 22.33 | 12528.1 |
| stage/render/camera_driver | 1.124 | 1.580 | 4.23 | 6329.5 |
| stage/render/prepare_write_phase_buffers | 0.769 | 1.065 | 3.44 | 4332.7 |
| bigspace/total/ms | 0.634 | 2.017 | 4.26 | 3570.8 |
| stage/render/prepare_assets | 0.570 | 0.741 | 10.54 | 3210.7 |
| render/present/ms | 0.492 | 0.618 | 2.34 | 2768.6 |
| stage/render/prepare_resources | 0.467 | 0.575 | 1.32 | 2630.8 |
| stage/main/first | 0.399 | 0.527 | 5.21 | 2247.5 |
| stage/render/prepare_meshes | 0.396 | 0.526 | 1.53 | 2231.2 |
| stage/main/pre_update | 0.340 | 0.501 | 1.53 | 1914.6 |
| terrain/update/ms | 0.312 | 0.413 | 2.33 | 1758.2 |
| stage/render/specialize | 0.290 | 0.425 | 3.35 | 1635.3 |
| controller/feed/ms | 0.285 | 0.369 | 0.75 | 1607.1 |
| terrain/select/ms | 0.278 | 0.367 | 0.85 | 1563.8 |
| stage/render/prepare_bind_groups | 0.267 | 0.450 | 2.57 | 1501.0 |
| bigspace/low_precision/ms | 0.263 | 0.938 | 3.33 | 1478.4 |
| stream/drive/ms | 0.255 | 0.347 | 20.09 | 1433.2 |
| stage/main/fixed_main_loop | 0.233 | 0.389 | 1.13 | 1312.3 |
| stage/render/queue | 0.219 | 0.374 | 3.56 | 1234.5 |
| bigspace/high_precision/ms | 0.214 | 1.048 | 3.21 | 1207.4 |
| anim/evaluate/ms | 0.204 | 0.344 | 2.61 | 1150.9 |
| stream/tick/ms | 0.196 | 0.269 | 0.57 | 1102.8 |
| far/select/ms | 0.193 | 0.268 | 0.80 | 1085.6 |
| stage/render/create_views | 0.189 | 0.249 | 0.93 | 1062.2 |
| stage/render/extract_commands | 0.181 | 0.219 | 0.53 | 1018.1 |
| stage/render/prepare_flush | 0.177 | 0.359 | 1.04 | 997.3 |
| stage/render/prepare_batch_phases | 0.163 | 0.232 | 0.58 | 919.8 |
| anim/evaluate_cpu/ms | 0.146 | 0.190 | 0.61 | 822.2 |
| stage/render/prepare_views | 0.136 | 0.175 | 0.39 | 767.1 |
| controller/move/ms | 0.132 | 0.251 | 0.57 | 744.9 |
| stage/main/last | 0.123 | 0.194 | 1.60 | 690.6 |
| render/acquire/ms | 0.094 | 0.123 | 0.32 | 527.8 |
| stage/render/phase_sort | 0.079 | 0.110 | 0.32 | 445.9 |
| pipelines/compile_ms | 0.052 | 0.000 | 121.24 | 294.0 |
| pipelines/front_ms | 0.050 | 0.000 | 117.26 | 282.4 |
| physics/step/ms | 0.050 | 0.087 | 0.59 | 280.3 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.077 | 0.31 | 275.2 |
| physics/insert/ms | 0.035 | 0.376 | 0.92 | 197.4 |
| encode/resolve_diagnostics/sum | 0.032 | 0.045 | 0.42 | 180.6 |
| stage/render/cleanup | 0.032 | 0.097 | 0.88 | 180.1 |
| weather/map/ms | 0.026 | 0.034 | 0.20 | 147.7 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.023 | 0.36 | 87.7 |
| anim/apply/ms | 0.012 | 0.021 | 0.06 | 65.6 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.35 | 63.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.19 | 60.0 |
| controller/avoid/ms | 0.010 | 0.014 | 0.10 | 58.3 |
| script/update/ms | 0.007 | 0.010 | 0.16 | 41.4 |
| script/ui/ms | 0.005 | 0.007 | 0.22 | 29.1 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 28.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.10 | 28.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.06 | 26.0 |
| controller/write/ms | 0.003 | 0.004 | 0.13 | 18.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 15.0 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.99 | 11.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.17 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 21 | 18 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1291 | 2262 | 641 |
| crowd/c0/meshes/count | count | 355 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5555 |
| crowd/c1/meshes/count | count | 1167 | 1295 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 237 |
| crowd/joints_extracted/count | count | 4622 | 10.1 k | 6494 |
| crowd/joints_uploaded/count | count | 4609 | 6835 | 6494 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.85 | 3.53 | 1.81 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.36 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.35 | 0.01 |
| encode/finish_critical | ms | 1.85 | 3.53 | 1.81 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.42 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.85 | 29.12 | 9.49 |
| gpu/timed/ms | ms | 8.64 | 10.50 | 8.83 |
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
| phases/alpha_mask3d/items | count | 819 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 819 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1285 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3418 | 5126 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1285 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3418 | 5126 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 201 | 19 |
| phases/shadow/bins | count | 4391 | 5558 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.2 k | 12.6 k | 5436 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 192 | 239 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 559 | 37 |
| preprocess/bind_groups_created | count | 14 | 559 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 132 | 211 | 123 |
| process/cpu_usage | % | 26.44 | 44.34 | 25.96 |
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
| started_load_count |  | 4919 | 5012 | 5012 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.21 | 58.53 | 32.16 |
| system/mem_usage | % | 68.18 | 68.63 | 68.53 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 65 | 248 | 69 |
| views/all/bins | count | 7036 | 8462 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 24.9 k | 13.0 k |
| views/all/pipelines | count | 237 | 286 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 22.3 k | 13.4 k |
| views/cascade/batch_sets | count | 10 | 33 | 19 |
| views/cascade/bins | count | 2219 | 2883 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7656 | 9646 | 5436 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 519 | 9493 | 5436 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2148 | 2779 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2403 | 3071 | 0 |
| views/lamp_face/pipelines | count | 168 | 208 | 0 |
| views/lamp_face/work_items | count | 0 | 2917 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2868 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8466 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8630 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 44.17 | 186 | 248.7 k |
| anim/events/count | count | 0.08 | 5 | 476 |
| assets/Image/added | count | 0.11 | 38 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.85 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1389 | 4675 |
| encode//count | count | 39.12 | 272 | 220.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5631 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5631 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5631 |
| physics/inserts | count | 0.40 | 95 | 2241 |
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
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6012 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1318 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.4 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.73 | 2727.3 | main/busy/ms 2696, stage/main/post_update 2595, stage/main/update 95.59, surface/advance_cpu_ms 67.06, stage/main/pre_update 2.45, stage/main/last 1.02, surface/cell_upload_ms 0.78, stage/main/first 0.49, stream/drive/ms 0.43, stage/main/fixed_main_loop 0.43, weather/map/ms 0.13, bigspace/total/ms 0.09 |
| warm-up | 3.47 | 739.1 | main/busy/ms 2872, stage/main/post_update 2618, render/thread/ms 706, render/busy/ms 581, stage/render/prepare_assets 495, stage/main/update 242, stage/render/prepare_views 124, render/acquire/ms 123, surface/advance_cpu_ms 67.06, stage/render/create_views 59.16, stage/render/prepare_resources 13.22, encode//sum 10.30 |
| warm-up | 3.53 | 64.3 | render/thread/ms 751, render/busy/ms 625, stage/render/prepare_assets 520, main/busy/ms 194, stage/main/update 150, stage/render/prepare_views 125, render/acquire/ms 123, stage/render/create_views 59.53, stage/main/post_update 33.99, stage/render/render 22.45, pipelines/compile_ms 19.16, terrain/update/ms 18.74 |
| warm-up | 3.58 | 45.3 | pipelines/compile_ms 147, pipelines/front_ms 126, pipelines/wait_ms 100, render/thread/ms 88.91, render/busy/ms 83.17, stage/render/render 47.32, render/graph/ms 39.35, main/busy/ms 38.73, stage/render/prepare_assets 26.28, terrain/update/ms 24.28, stage/main/post_update 24.09, pipelines/driver_ms 20.79 |
| warm-up | 3.73 | 152.2 | pipelines/wait_ms 5523, pipelines/compile_ms 1519, pipelines/front_ms 1425, render/thread/ms 183, render/busy/ms 177, pipelines/driver_ms 94.17, main/busy/ms 69.44, stage/render/prepare_resources 68.63, stage/render/render 40.16, stage/main/post_update 33.94, render/graph/ms 32.68, stage/render/prepare_write_phase_buffers 30.58 |
| warm-up | 3.80 | 69.6 | pipelines/wait_ms 6852, pipelines/compile_ms 2892, pipelines/front_ms 2766, render/thread/ms 207, render/busy/ms 204, main/busy/ms 132, pipelines/driver_ms 126, stage/render/prepare_resources 86.44, stage/main/update 62.36, encode//sum 41.62, stage/render/prepare_write_phase_buffers 32.73, stage/main/post_update 31.89 |
| warm-up | 3.92 | 117.7 | pipelines/compile_ms 1915, pipelines/front_ms 1807, pipelines/wait_ms 1426, render/thread/ms 164, render/busy/ms 135, pipelines/driver_ms 107, main/busy/ms 105, stage/render/prepare_assets 73.54, stage/main/update 66.01, stage/render/render 48.98, encode//sum 41.74, render/present/ms 28.63 |
| warm-up | 3.96 | 42.7 | pipelines/compile_ms 823, pipelines/front_ms 681, pipelines/driver_ms 142, render/thread/ms 137, render/busy/ms 108, stage/render/prepare_assets 73.97, stage/render/render 55.08, main/busy/ms 40.58, render/present/ms 28.44, pipelines/wait_ms 27.08, render/graph/ms 25.66, stage/render/graph_submit 20.83 |
| warm-up | 4.01 | 30.2 | pipelines/compile_ms 999, pipelines/front_ms 933, pipelines/wait_ms 306, pipelines/driver_ms 66.34, render/thread/ms 46.26, render/busy/ms 44.40, main/busy/ms 41.86, stage/main/post_update 23.32, stage/render/prepare_resources 13.36, stage/render/prepare_assets 12.49, stage/render/render 10.93, encode//sum 9.12 |
| warm-up | 4.17 | 91.5 | pipelines/compile_ms 980, pipelines/front_ms 934, main/busy/ms 54.69, render/thread/ms 51.27, render/busy/ms 49.69, pipelines/driver_ms 46.17, stage/main/update 42.69, stream/drive/ms 36.39, stage/render/prepare_assets 34.64, stage/render/render 10.63, render/graph/ms 8.72, stage/main/post_update 8.00 |
| warm-up | 4.20 | 32.0 | pipelines/compile_ms 1276, pipelines/front_ms 1223, render/thread/ms 69.08, render/busy/ms 66.86, pipelines/driver_ms 52.96, stage/render/prepare_assets 38.94, main/busy/ms 35.73, stage/main/update 19.65, stage/render/render 13.99, stream/drive/ms 12.40, render/graph/ms 11.50, stage/main/post_update 10.03 |
| warm-up | 4.23 | 31.8 | pipelines/compile_ms 561, pipelines/front_ms 538, render/thread/ms 55.15, render/busy/ms 52.67, main/busy/ms 38.47, pipelines/driver_ms 23.52, stage/render/prepare_assets 20.68, stage/main/update 18.83, stage/render/render 17.91, render/graph/ms 15.13, stream/drive/ms 13.03, stage/main/post_update 9.95 |
| warm-up | 4.26 | 29.1 | pipelines/compile_ms 288, pipelines/front_ms 277, render/thread/ms 53.98, main/busy/ms 49.57, render/busy/ms 48.31, stage/main/update 30.92, stage/render/prepare_assets 27.19, stream/drive/ms 25.02, stage/render/render 19.99, render/graph/ms 13.95, pipelines/driver_ms 11.06, stage/main/post_update 10.27 |
| warm-up | 4.33 | 70.2 | pipelines/compile_ms 279, pipelines/front_ms 269, render/thread/ms 70.07, render/busy/ms 64.59, main/busy/ms 48.26, stage/render/prepare_assets 43.12, stage/main/update 31.33, stream/drive/ms 26.12, stage/render/render 17.87, render/graph/ms 11.71, pipelines/driver_ms 10.12, stage/main/post_update 9.97 |
| warm-up | 4.39 | 64.4 | pipelines/compile_ms 683, pipelines/front_ms 660, render/thread/ms 101, render/busy/ms 97.41, main/busy/ms 51.06, stage/render/prepare_assets 40.80, stage/main/update 25.52, pipelines/driver_ms 23.51, stage/render/render 18.79, encode//sum 16.67, render/graph/ms 16.29, stage/render/prepare_flush 14.24 |
| warm-up | 4.51 | 122.1 | pipelines/compile_ms 1762, pipelines/front_ms 1703, render/thread/ms 143, render/busy/ms 109, main/busy/ms 83.00, pipelines/driver_ms 59.26, stage/main/update 56.15, stage/render/render 51.20, stage/render/prepare_assets 50.28, render/present/ms 32.14, render/graph/ms 18.20, stage/render/prepare_flush 14.20 |
| warm-up | 4.59 | 79.3 | pipelines/compile_ms 2584, pipelines/front_ms 2395, pipelines/driver_ms 188, render/thread/ms 148, render/busy/ms 116, pipelines/wait_ms 106, stage/render/prepare_assets 86.40, main/busy/ms 73.42, stage/render/render 47.24, stage/main/update 43.79, render/present/ms 32.13, stage/main/post_update 20.11 |
| warm-up | 4.67 | 73.3 | pipelines/compile_ms 2215, pipelines/front_ms 2043, pipelines/driver_ms 172, pipelines/wait_ms 144, render/thread/ms 124, render/busy/ms 122, main/busy/ms 68.31, stage/render/prepare_assets 62.14, stage/main/update 37.19, stage/render/prepare_resources 24.72, stage/main/post_update 22.74, encode//sum 21.07 |
| warm-up | 4.76 | 91.9 | pipelines/compile_ms 2025, pipelines/front_ms 1950, pipelines/wait_ms 1273, render/thread/ms 151, render/busy/ms 148, main/busy/ms 78.56, pipelines/driver_ms 74.61, encode//sum 71.47, stage/render/prepare_resources 65.96, stage/main/update 39.52, stage/main/post_update 33.44, stage/render/render 29.58 |
| warm-up | 4.82 | 62.8 | pipelines/compile_ms 2045, pipelines/front_ms 1969, pipelines/wait_ms 1415, render/thread/ms 147, render/busy/ms 139, encode//sum 99.93, pipelines/driver_ms 76.26, main/busy/ms 71.31, stage/render/prepare_resources 56.46, stage/render/render 41.70, render/graph/ms 39.19, stage/main/post_update 35.94 |
| warm-up | 4.86 | 34.7 | pipelines/compile_ms 1031, pipelines/front_ms 999, pipelines/wait_ms 242, render/thread/ms 86.50, render/busy/ms 78.58, encode//sum 49.16, main/busy/ms 48.44, pipelines/driver_ms 31.82, stage/render/render 29.28, render/graph/ms 26.86, stage/main/update 25.38, stage/render/graph_submit 21.22 |
| warm-up | 4.97 | 83.8 | pipelines/compile_ms 1657, pipelines/front_ms 1610, pipelines/wait_ms 541, main/busy/ms 94.56, stage/main/update 69.43, render/thread/ms 57.30, render/busy/ms 55.03, stream/drive/ms 48.01, pipelines/driver_ms 46.77, stage/render/render 17.30, encode//sum 17.23, stage/main/post_update 14.76 |
| warm-up | 5.02 | 52.8 | pipelines/compile_ms 1622, pipelines/front_ms 1570, pipelines/wait_ms 722, main/busy/ms 115, stage/main/update 90.98, render/thread/ms 77.41, render/busy/ms 74.86, pipelines/driver_ms 52.06, stream/drive/ms 50.56, stage/render/prepare_assets 26.45, stage/render/render 17.46, encode//sum 16.87 |
| warm-up | 5.17 | 154.2 | pipelines/compile_ms 1556, pipelines/front_ms 1505, pipelines/wait_ms 298, render/thread/ms 124, render/busy/ms 122, main/busy/ms 105, stage/render/prepare_assets 92.18, stage/main/update 83.22, pipelines/driver_ms 51.11, stream/drive/ms 50.11, encode//sum 15.06, stage/render/render 12.90 |
| warm-up | 5.22 | 42.5 | pipelines/compile_ms 1213, pipelines/front_ms 1171, render/thread/ms 122, render/busy/ms 121, main/busy/ms 89.06, stage/render/prepare_assets 82.85, stage/main/update 63.92, stream/drive/ms 43.39, pipelines/driver_ms 42.05, stage/render/render 25.34, render/graph/ms 23.23, stage/render/graph_submit 20.33 |
| warm-up | 5.29 | 74.7 | pipelines/compile_ms 123, pipelines/front_ms 117, main/busy/ms 83.32, render/thread/ms 68.84, render/busy/ms 67.15, stage/main/update 62.60, stream/drive/ms 43.37, stage/render/prepare_assets 30.43, stage/render/render 23.62, render/graph/ms 21.47, stage/render/graph_submit 18.77, encode/submit 15.79 |
| warm-up | 5.37 | 54.7 | pipelines/compile_ms 1122, pipelines/front_ms 1091, main/busy/ms 60.30, stage/main/update 45.76, render/thread/ms 35.11, render/busy/ms 33.64, stream/drive/ms 33.52, pipelines/driver_ms 30.63, encode//sum 15.93, stage/render/render 12.31, stage/render/prepare_assets 10.29, render/graph/ms 10.27 |
| warm-up | 5.44 | 28.6 | pipelines/compile_ms 202, pipelines/front_ms 194, render/thread/ms 40.88, render/busy/ms 39.28, main/busy/ms 33.72, encode//sum 16.48, stage/main/post_update 15.88, stage/render/render 13.66, stage/main/update 12.08, render/graph/ms 11.72, stage/render/prepare_assets 8.44, stage/render/graph_submit 8.36 |
| warm-up | 5.53 | 39.8 | pipelines/compile_ms 136, pipelines/front_ms 132, render/thread/ms 56.37, render/busy/ms 54.82, main/busy/ms 40.32, stage/render/prepare_assets 25.38, stage/main/update 23.46, encode//sum 16.41, stage/render/render 15.05, render/graph/ms 12.71, stage/main/post_update 10.69, stage/render/graph_submit 9.49 |
| warm-up | 5.56 | 29.4 | pipelines/compile_ms 742, pipelines/front_ms 719, render/thread/ms 55.43, render/busy/ms 53.82, main/busy/ms 34.39, stage/render/prepare_assets 29.63, pipelines/driver_ms 22.68, stage/main/update 19.50, encode//sum 17.08, stage/render/render 14.66, render/graph/ms 12.30, stage/render/graph_submit 9.58 |
| warm-up | 5.65 | 40.2 | pipelines/compile_ms 586, pipelines/front_ms 544, render/thread/ms 60.86, render/busy/ms 58.96, main/busy/ms 55.74, pipelines/driver_ms 41.26, stage/main/update 23.33, stage/render/render 16.86, encode//sum 16.10, stage/main/post_update 15.88, render/graph/ms 14.35, stage/render/prepare_resources 13.49 |
| warm-up | 5.70 | 47.7 | pipelines/compile_ms 342, pipelines/front_ms 310, render/thread/ms 75.48, render/busy/ms 73.93, main/busy/ms 56.50, pipelines/driver_ms 31.64, stage/render/prepare_assets 30.00, pipelines/wait_ms 29.73, stage/main/update 26.82, encode//sum 14.54, stage/render/render 13.42, stage/render/prepare_resources 13.15 |
| warm-up | 5.78 | 31.6 | pipelines/compile_ms 409, pipelines/front_ms 397, render/thread/ms 47.58, main/busy/ms 46.91, render/busy/ms 45.85, stage/main/update 26.68, stage/render/prepare_assets 20.17, encode//sum 18.85, stage/render/render 13.96, stream/drive/ms 12.75, stage/main/post_update 12.68, pipelines/driver_ms 11.64 |
| warm-up | 5.81 | 33.5 | pipelines/compile_ms 1241, pipelines/front_ms 1206, render/thread/ms 53.13, render/busy/ms 51.29, main/busy/ms 45.11, pipelines/driver_ms 34.76, stage/main/update 24.27, stage/render/prepare_assets 23.42, encode//sum 18.68, stage/render/render 14.60, stage/main/post_update 12.75, render/graph/ms 11.70 |
| warm-up | 5.84 | 34.7 | pipelines/compile_ms 1043, pipelines/front_ms 1016, render/thread/ms 54.50, render/busy/ms 53.05, stage/render/prepare_assets 31.97, main/busy/ms 30.06, pipelines/driver_ms 27.89, encode//sum 15.38, stage/main/post_update 12.83, stage/render/render 10.65, stage/main/update 9.73, render/graph/ms 8.52 |
| warm-up | 6.38 | 29.3 | render/thread/ms 37.14, render/busy/ms 36.01, main/busy/ms 26.35, stage/render/prepare_assets 16.06, encode//sum 14.65, stage/main/update 12.11, stage/main/post_update 8.63, stage/render/render 8.54, render/graph/ms 6.94, stage/render/graph_submit 5.01, stream/drive/ms 4.55, stage/render/specialize 3.25 |
| warm-up | 6.71 | 33.1 | main/busy/ms 42.05, render/thread/ms 36.26, render/busy/ms 34.58, stage/main/update 23.64, encode//sum 21.32, stage/render/render 17.14, stream/drive/ms 14.89, render/graph/ms 14.63, stage/main/post_update 13.56, stage/render/graph_submit 11.35, encode/submit 6.60, stage/render/prepare_assets 5.65 |
| warm-up | 6.77 | 28.4 | pipelines/compile_ms 155, pipelines/front_ms 151, render/thread/ms 43.16, render/busy/ms 41.85, main/busy/ms 38.41, stage/main/update 22.47, stage/render/prepare_assets 21.16, encode//sum 19.65, stage/render/render 10.76, stage/main/post_update 10.34, render/graph/ms 9.00, stage/render/graph_submit 6.53 |
| warm-up | 7.13 | 32.3 | main/busy/ms 41.53, render/thread/ms 26.63, render/busy/ms 25.52, encode//sum 23.74, stage/main/update 22.78, stream/drive/ms 15.32, stream/spawn/ms 13.92, stage/render/render 13.22, stage/main/post_update 12.69, render/graph/ms 11.75, stage/render/graph_submit 9.10, encode/finish_wall 5.09 |
| measure | 0.78 | 28.5 | main/busy/ms 34.39, render/thread/ms 25.34, render/busy/ms 23.83, stage/main/update 18.82, encode//sum 18.62, stream/drive/ms 14.08, stage/render/render 12.42, stage/main/post_update 12.40, render/graph/ms 10.60, stage/render/graph_submit 7.69, stage/render/prepare_assets 6.19, encode/submit 4.19 |
| measure | 18.18 | 30.8 | main/busy/ms 39.79, stage/main/update 24.39, render/thread/ms 22.30, encode//sum 21.50, render/busy/ms 21.20, stream/drive/ms 20.32, stage/render/render 13.83, stage/main/post_update 12.75, render/graph/ms 12.27, stage/render/graph_submit 9.25, encode/submit 5.18, encode/finish_wall 4.16 |
| measure | 59.05 | 29.0 | render/thread/ms 41.10, render/busy/ms 39.94, main/busy/ms 30.68, encode//sum 22.89, stage/main/post_update 17.48, stage/render/prepare_assets 14.48, stage/render/render 13.85, render/graph/ms 12.34, stage/render/graph_submit 9.66, stage/main/update 7.77, encode/finish_wall 5.07, encode/submit 4.66 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_on\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 8% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.1 W · 72 °C (max 78) · VRAM 5972 MiB · throttled 100% of samples
