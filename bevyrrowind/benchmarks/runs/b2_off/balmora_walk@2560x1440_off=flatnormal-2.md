# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: flatnormal · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5248 frames over 65.7 s · **median 12.20 ms (82 fps)** · mean 12.51 · p95 15.46 · p99 17.73 (1% low 56 fps) · worst 35.74 ms · 39 spikes over 1.5× median · 26.5 stutters/min (work 26.5/min)
- warm-up: 370 frames over 9.0 s, worst 2815.2 ms
- hitches (> 30.5 ms): **6** measured, 22 in warm-up
- timed GPU passes: 7.73 ms of a 12.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.20 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.50/min, limit 1/min
- **FAIL** worst frame 35.74 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.194 | 2.274 | 2.717 | 0.96 | 0.183 |
| main_opaque_pass_3d | 1.743 | 1.806 | 2.571 | 0.96 | 0.062 |
| early prepass | 0.533 | 0.552 | 0.916 | 0.96 | 0.088 |
| bloom | 0.336 | 0.348 | 0.590 | 0.96 | 0.068 |
| atmosphere_luts | 0.326 | 0.338 | 0.562 | 0.96 | 0.004 |
| water_velocity | 0.307 | 0.318 | 0.601 | 0.96 | 0.002 |
| water_sspr | 0.223 | 0.231 | 0.456 | 0.96 | 0.003 |
| ssao | 0.198 | 0.206 | 0.348 | 0.96 | 0.008 |
| uniform_allocation | 0.195 | 0.202 | 0.205 | 0.96 | 0.035 |
| atmos_froxel_inject | 0.159 | 0.165 | 0.386 | 0.96 | 0.002 |
| bin_unpacking | 0.151 | 0.157 | 0.324 | 0.96 | 0.025 |
| auxiliary_views | 0.143 | 0.148 | 0.618 | 0.96 | 0.138 |
| clouds_trace | 0.136 | 0.141 | 0.234 | 0.96 | 0.001 |
| water_fft | 0.135 | 0.140 | 0.139 | 0.96 | 0.003 |
| upscaling | 0.129 | 0.134 | 0.168 | 0.96 | 0.005 |
| clouds_composite | 0.110 | 0.114 | 0.111 | 0.96 | 0.002 |
| main_transmissive_pass_3d | 0.092 | 0.095 | 0.180 | 0.96 | 0.009 |
| tonemapping | 0.091 | 0.095 | 0.092 | 0.96 | 0.004 |
| clustering | 0.070 | 0.072 | 0.071 | 0.96 | 0.021 |
| render_sky | 0.053 | 0.055 | 0.054 | 0.96 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.452 | 0.676 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.96 | 0.001 |
| surface_trail | 0.047 | 0.061 | 0.058 | 0.77 | 0.002 |
| early_mesh_preprocessing | 0.045 | 0.046 | 0.049 | 0.96 | 0.005 |
| water_snapshot | 0.037 | 0.039 | 0.040 | 0.96 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.231 | 0.237 | 0.11 | 0.005 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.023 | 0.96 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.96 | 0.117 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.96 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.96 | 0.002 |
| sparse buffer updates | 0.008 | 0.008 | 0.008 | 0.96 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.96 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 17.086 | 0.000 | 9057.38 | 89667.4 |
| encode//sum | 11.445 | 16.957 | 87.37 | 60064.5 |
| render/thread/ms | 11.010 | 13.835 | 30.65 | 57779.5 |
| render/busy/ms | 10.401 | 13.225 | 29.96 | 54583.0 |
| main/busy/ms | 9.680 | 12.197 | 29.75 | 50800.0 |
| render/graph/ms | 6.320 | 8.298 | 18.82 | 33165.2 |
| stage/main/post_update | 6.061 | 8.166 | 14.79 | 31808.9 |
| stage/render/graph_submit | 5.085 | 6.713 | 17.23 | 26686.0 |
| stage/render/render | 4.373 | 8.462 | 16.27 | 22952.1 |
| stage/render/cleanup | 2.778 | 8.465 | 19.83 | 14578.5 |
| encode/finish_wall | 2.761 | 3.743 | 11.30 | 14489.6 |
| encode/submit | 2.399 | 3.426 | 6.67 | 12587.9 |
| stage/main/update | 2.276 | 2.839 | 23.36 | 11945.9 |
| stage/render/camera_driver | 1.174 | 1.619 | 4.60 | 6161.2 |
| stage/render/prepare_write_phase_buffers | 0.964 | 1.299 | 2.61 | 5057.4 |
| bigspace/total/ms | 0.595 | 1.914 | 3.98 | 3122.7 |
| stage/render/prepare_assets | 0.591 | 0.786 | 10.44 | 3102.5 |
| render/present/ms | 0.510 | 0.697 | 2.10 | 2674.5 |
| stage/render/prepare_resources | 0.460 | 0.605 | 1.53 | 2414.3 |
| stage/main/first | 0.422 | 0.563 | 5.43 | 2213.7 |
| stage/render/prepare_meshes | 0.395 | 0.559 | 2.43 | 2070.6 |
| pipelines/compile_ms | 0.378 | 0.000 | 160.84 | 1981.8 |
| pipelines/front_ms | 0.365 | 0.000 | 155.65 | 1913.1 |
| stage/main/pre_update | 0.349 | 0.542 | 1.31 | 1830.6 |
| terrain/update/ms | 0.314 | 0.402 | 2.02 | 1645.7 |
| controller/feed/ms | 0.284 | 0.384 | 0.77 | 1489.9 |
| stage/render/specialize | 0.282 | 0.430 | 2.47 | 1480.9 |
| terrain/select/ms | 0.279 | 0.358 | 1.19 | 1464.7 |
| stage/render/queue | 0.275 | 0.464 | 2.53 | 1444.6 |
| stream/drive/ms | 0.271 | 0.394 | 21.27 | 1421.9 |
| stage/render/prepare_bind_groups | 0.270 | 0.487 | 2.78 | 1416.5 |
| stage/main/fixed_main_loop | 0.257 | 0.425 | 1.37 | 1346.6 |
| bigspace/low_precision/ms | 0.250 | 0.712 | 3.53 | 1314.2 |
| stream/tick/ms | 0.209 | 0.298 | 0.80 | 1098.7 |
| far/select/ms | 0.204 | 0.294 | 0.69 | 1072.5 |
| stage/render/create_views | 0.196 | 0.270 | 7.71 | 1030.7 |
| stage/render/prepare_batch_phases | 0.187 | 0.274 | 1.20 | 982.7 |
| bigspace/high_precision/ms | 0.177 | 0.622 | 2.80 | 927.5 |
| stage/render/prepare_flush | 0.166 | 0.333 | 0.88 | 870.2 |
| anim/evaluate_cpu/ms | 0.163 | 0.210 | 0.58 | 854.1 |
| stage/render/extract_commands | 0.161 | 0.203 | 0.44 | 845.1 |
| anim/evaluate/ms | 0.147 | 0.270 | 3.05 | 772.0 |
| stage/render/prepare_views | 0.145 | 0.201 | 0.47 | 758.4 |
| controller/move/ms | 0.141 | 0.305 | 0.74 | 737.8 |
| stage/main/last | 0.123 | 0.221 | 1.97 | 647.6 |
| render/acquire/ms | 0.099 | 0.142 | 0.42 | 522.0 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.133 | 0.35 | 324.6 |
| physics/step/ms | 0.056 | 0.093 | 0.82 | 291.3 |
| physics/insert/ms | 0.048 | 0.419 | 0.95 | 252.8 |
| stage/render/phase_sort | 0.034 | 0.061 | 0.42 | 180.5 |
| encode/resolve_diagnostics/sum | 0.032 | 0.045 | 0.47 | 170.5 |
| weather/map/ms | 0.025 | 0.034 | 0.30 | 131.4 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.024 | 0.42 | 83.8 |
| pipelines/driver_ms | 0.013 | 0.000 | 5.29 | 68.7 |
| anim/apply/ms | 0.012 | 0.021 | 0.22 | 62.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.28 | 60.1 |
| controller/avoid/ms | 0.011 | 0.015 | 0.26 | 59.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.43 | 56.3 |
| script/update/ms | 0.008 | 0.011 | 0.07 | 40.0 |
| stream/despawn/ms | 0.006 | 0.000 | 1.27 | 30.1 |
| script/ui/ms | 0.005 | 0.007 | 0.25 | 28.7 |
| script/input/ms | 0.005 | 0.007 | 0.14 | 28.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.04 | 26.8 |
| controller/write/ms | 0.004 | 0.005 | 0.03 | 18.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 15.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.23 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.12 | 3.4 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 15 | 0 |
| anim/a2/count | count | 1 | 16 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 39 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1301 | 2262 | 659 |
| crowd/c0/meshes/count | count | 353 | 646 | 171 |
| crowd/c1/count | count | 145 | 155 | 147 |
| crowd/c1/joints/count | count | 5507 | 5936 | 5537 |
| crowd/c1/meshes/count | count | 1180 | 1268 | 1170 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 17.3 k | 6179 |
| crowd/joints_uploaded/count | count | 4249 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.27 | 9.24 | 2.22 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.42 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.02 |
| encode/finish_critical | ms | 2.27 | 9.24 | 2.22 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.47 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.33 | 29.96 | 10.09 |
| gpu/timed/ms | ms | 8.70 | 11.10 | 9.40 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 545.28 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 193 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 833 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 193 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 833 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1286 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3464 | 5113 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 1286 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3464 | 5113 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 19 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 228 | 28 |
| phases/shadow/bins | count | 4825 | 5829 | 2221 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.7 k | 13.1 k | 5676 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 256 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2647 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 1025 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 183 | 185 | 185 |
| pipelines/waiting | count | 0 | 21 | 0 |
| preprocess/bind_groups | count | 11 | 627 | 10 |
| preprocess/bind_groups_created | count | 11 | 627 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 191 | 288 | 181 |
| process/cpu_usage | % | 26.13 | 40.36 | 15.87 |
| process/mem_usage | GiB | 5.12 | 5.30 | 5.30 |
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
| started_load_count |  | 4768 | 4862 | 4862 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.68 | 58.44 | 37.52 |
| system/mem_usage | % | 68.79 | 69.25 | 69.07 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2934 | 2934 |
| textures/images | count | 2589 | 2644 | 2644 |
| views/all/batch_sets | count | 90 | 281 | 84 |
| views/all/bins | count | 7524 | 8620 | 5719 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.7 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 255 | 305 | 80 |
| views/all/pipelines_distinct | count | 74 | 79 | 67 |
| views/all/work_items | count | 17.3 k | 23.1 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2608 | 3169 | 2221 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8162 | 10.2 k | 5676 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8162 | 10.2 k | 5676 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2154 | 2776 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2412 | 3097 | 0 |
| views/lamp_face/pipelines | count | 168 | 213 | 0 |
| views/lamp_face/work_items | count | 0 | 2862 | 0 |
| views/main/batch_sets | count | 55 | 58 | 56 |
| views/main/bins | count | 2878 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8588 | 13.0 k | 7618 |
| views/main/pipelines | count | 51 | 54 | 52 |
| views/main/work_items | count | 8743 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 54.68 | 562 | 286.9 k |
| anim/events/count | count | 0.12 | 5 | 610 |
| assets/Image/added | count | 0.12 | 42 | 652 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.82 | 167 | 4308 |
| assets/Mesh/removed | count | 0.93 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 529 |
| assets/StandardMaterial/modified | count | 4.13 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.69 | 1805 | 8869 |
| encode//count | count | 38.63 | 272 | 202.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5248 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5248 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5248 |
| physics/inserts | count | 0.63 | 145 | 3304 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 145 | 2335 |
| physics/steps | count | 0.75 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 3 | 11 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6451 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1414 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.5 ms; slowest 28, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.82 | 2815.2 | main/busy/ms 2786, stage/main/post_update 2684, stage/main/update 96.75, surface/advance_cpu_ms 66.60, stage/main/pre_update 2.56, stage/main/last 1.03, surface/cell_upload_ms 0.80, stage/main/first 0.50, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.34, weather/map/ms 0.21, stream/tick/ms 0.11 |
| warm-up | 3.54 | 720.2 | main/busy/ms 2824, stage/main/post_update 2711, render/thread/ms 676, render/busy/ms 517, stage/render/prepare_assets 437, stage/render/prepare_views 157, render/acquire/ms 157, stage/main/update 101, surface/advance_cpu_ms 66.60, stage/render/prepare_resources 51.97, stage/render/create_views 19.83, stage/render/cleanup 7.93 |
| warm-up | 3.77 | 235.8 | pipelines/wait_ms 2530, render/thread/ms 687, render/busy/ms 526, stage/render/prepare_assets 437, pipelines/compile_ms 246, pipelines/front_ms 229, stage/render/prepare_views 158, render/acquire/ms 158, main/busy/ms 55.19, stage/render/prepare_resources 53.74, stage/main/post_update 37.53, stage/render/create_views 20.04 |
| warm-up | 4.02 | 94.9 | main/busy/ms 65.64, stage/main/update 53.64, render/thread/ms 53.54, render/busy/ms 51.80, stage/render/prepare_assets 40.98, stage/main/post_update 9.18, stage/render/render 8.18, render/graph/ms 5.68, stage/render/graph_submit 2.81, stage/render/camera_driver 2.72, encode/submit 2.31, encode//sum 1.80 |
| warm-up | 4.16 | 47.5 | render/thread/ms 43.57, render/busy/ms 42.11, main/busy/ms 38.72, stage/render/prepare_assets 32.95, stage/main/update 31.47, stream/drive/ms 28.59, stage/render/render 5.73, stage/main/post_update 4.35, render/graph/ms 3.73, stage/render/graph_submit 2.20, encode/submit 1.85, encode//sum 1.49 |
| warm-up | 4.21 | 44.3 | render/thread/ms 36.43, render/busy/ms 34.15, main/busy/ms 31.67, stage/render/prepare_assets 21.77, stage/main/update 20.20, stream/drive/ms 16.23, stage/main/post_update 7.76, stage/render/render 7.49, render/graph/ms 4.56, stage/render/graph_submit 2.66, encode/submit 2.25, stage/render/prepare_resources 1.94 |
| warm-up | 4.29 | 49.2 | pipelines/wait_ms 3401, pipelines/compile_ms 243, pipelines/front_ms 220, render/thread/ms 56.46, render/busy/ms 54.11, main/busy/ms 49.82, stage/render/prepare_assets 35.49, stage/main/update 29.26, stream/drive/ms 24.43, pipelines/driver_ms 23.68, stage/main/post_update 13.13, stage/render/cleanup 11.52 |
| warm-up | 4.33 | 41.8 | pipelines/wait_ms 3235, pipelines/compile_ms 69.73, render/thread/ms 69.69, render/busy/ms 67.36, pipelines/front_ms 65.62, stage/render/prepare_assets 46.14, main/busy/ms 42.44, stage/main/update 22.49, stream/drive/ms 17.30, stage/main/post_update 11.47, render/graph/ms 9.13, stage/render/cleanup 7.28 |
| warm-up | 4.43 | 68.4 | pipelines/wait_ms 1592, pipelines/compile_ms 61.35, pipelines/front_ms 59.07, main/busy/ms 51.36, render/thread/ms 47.21, render/busy/ms 45.70, stage/main/update 37.55, stream/drive/ms 32.48, stage/render/prepare_assets 30.18, stage/main/post_update 8.70, render/graph/ms 5.08, encode//sum 4.62 |
| warm-up | 4.47 | 39.9 | render/thread/ms 63.80, render/busy/ms 61.64, main/busy/ms 28.27, stage/render/prepare_assets 26.14, encode//sum 17.60, stage/main/update 13.14, stage/render/create_views 12.23, render/graph/ms 9.33, stage/main/post_update 8.44, stream/drive/ms 7.71, stage/render/cleanup 7.42, stage/render/graph_submit 5.37 |
| warm-up | 4.53 | 37.7 | pipelines/wait_ms 6402, pipelines/compile_ms 188, pipelines/front_ms 175, render/thread/ms 51.89, render/busy/ms 49.80, main/busy/ms 24.13, encode//sum 20.40, stage/render/prepare_assets 17.88, stage/render/render 13.54, pipelines/driver_ms 13.06, render/graph/ms 10.86, stage/main/update 9.39 |
| warm-up | 4.65 | 82.7 | main/busy/ms 69.76, stage/main/update 52.69, stream/drive/ms 47.18, render/thread/ms 41.56, render/busy/ms 40.34, stage/render/prepare_assets 18.71, stage/main/post_update 8.66, stage/render/render 6.12, render/graph/ms 4.51, stage/render/specialize 4.30, encode//sum 3.99, stream/spawn/ms 3.36 |
| warm-up | 4.68 | 34.4 | pipelines/wait_ms 875, main/busy/ms 76.37, stage/main/update 58.41, stream/drive/ms 52.60, render/thread/ms 51.82, render/busy/ms 50.40, stage/render/prepare_assets 23.04, stage/main/post_update 9.48, stage/render/render 7.63, stage/render/specialize 6.14, render/graph/ms 5.57, encode//sum 4.96 |
| warm-up | 4.76 | 76.9 | pipelines/wait_ms 4061, pipelines/compile_ms 191, pipelines/front_ms 174, main/busy/ms 76.51, stage/main/update 57.23, stream/drive/ms 51.29, render/thread/ms 47.89, render/busy/ms 46.54, stage/render/prepare_assets 30.87, pipelines/driver_ms 17.29, stage/main/post_update 13.34, stage/render/render 6.37 |
| warm-up | 4.83 | 43.6 | main/busy/ms 49.12, stage/main/update 38.89, stream/drive/ms 35.61, render/thread/ms 32.40, render/busy/ms 30.89, stage/render/prepare_assets 13.00, stage/main/post_update 5.72, render/graph/ms 5.03, encode//sum 4.56, stage/render/cleanup 4.02, stage/render/specialize 3.26, stage/render/graph_submit 2.98 |
| warm-up | 4.87 | 37.2 | main/busy/ms 38.07, stage/main/update 26.84, stream/drive/ms 22.53, render/thread/ms 17.89, render/busy/ms 16.75, stage/main/post_update 8.14, stage/render/prepare_assets 5.27, render/graph/ms 4.23, stage/render/cleanup 3.46, encode//sum 3.17, stage/render/graph_submit 2.71, stage/render/render 2.33 |
| warm-up | 5.32 | 37.7 | pipelines/wait_ms 2948, pipelines/compile_ms 78.04, pipelines/front_ms 74.68, main/busy/ms 45.63, render/thread/ms 45.53, render/busy/ms 43.51, stage/main/update 30.65, stage/render/prepare_assets 22.35, stage/render/render 10.95, stage/main/post_update 10.72, render/graph/ms 8.31, encode//sum 6.77 |
| warm-up | 5.84 | 35.4 | pipelines/wait_ms 3991, pipelines/compile_ms 76.21, pipelines/front_ms 71.55, main/busy/ms 35.94, render/thread/ms 32.09, render/busy/ms 30.51, stage/main/update 23.06, stage/render/prepare_assets 13.28, stream/drive/ms 10.48, stage/render/render 8.55, stage/main/post_update 8.47, encode//sum 7.71 |
| warm-up | 5.94 | 31.5 | render/thread/ms 38.58, render/busy/ms 37.16, main/busy/ms 31.26, stage/render/prepare_assets 21.80, stage/main/update 14.03, stage/main/post_update 10.76, encode//sum 6.20, stage/render/render 6.09, stream/drive/ms 5.78, render/graph/ms 4.34, stage/render/graph_submit 2.81, stage/render/prepare_write_phase_buffers 2.35 |
| warm-up | 6.13 | 31.8 | render/thread/ms 33.49, main/busy/ms 32.88, render/busy/ms 32.35, stage/main/update 20.06, stage/render/prepare_assets 19.46, stream/drive/ms 16.10, stage/main/post_update 9.27, encode//sum 6.92, stage/render/render 6.42, render/graph/ms 4.83, stage/render/graph_submit 3.32, encode/submit 2.19 |
| warm-up | 7.16 | 35.3 | main/busy/ms 44.39, render/thread/ms 35.70, render/busy/ms 34.03, stage/main/update 23.85, stage/main/post_update 17.18, stage/render/prepare_assets 14.01, encode//sum 11.27, stage/render/render 10.96, render/graph/ms 8.46, stage/render/graph_submit 5.49, terrain/update/ms 4.74, encode/submit 3.39 |
| warm-up | 7.26 | 44.7 | pipelines/wait_ms 8194, pipelines/compile_ms 150, pipelines/front_ms 146, render/thread/ms 39.77, main/busy/ms 39.53, render/busy/ms 37.93, stage/main/update 25.76, stage/render/prepare_assets 18.44, encode//sum 13.46, stage/render/render 11.59, stage/main/post_update 9.58, render/graph/ms 8.79 |
| measure | 0.78 | 35.7 | pipelines/wait_ms 9057, pipelines/compile_ms 133, pipelines/front_ms 128, main/busy/ms 38.71, render/thread/ms 32.01, render/busy/ms 30.69, stage/main/update 22.83, encode//sum 21.98, stage/render/render 14.80, stream/drive/ms 13.23, render/graph/ms 12.92, stage/main/post_update 12.17 |
| measure | 0.92 | 31.0 | pipelines/wait_ms 10.8 k, main/busy/ms 38.64, render/thread/ms 35.28, render/busy/ms 34.17, encode//sum 29.69, stage/main/update 24.30, pipelines/compile_ms 17.19, stage/render/render 17.04, pipelines/front_ms 16.13, render/graph/ms 15.22, stage/main/post_update 11.64, stage/render/graph_submit 11.50 |
| measure | 2.73 | 31.8 | encode//sum 50.38, render/thread/ms 41.84, render/busy/ms 40.56, render/graph/ms 21.24, main/busy/ms 18.01, stage/render/graph_submit 15.89, stage/render/render 15.74, stage/main/post_update 10.67, encode/submit 8.64, stage/render/create_views 7.86, stage/render/cleanup 7.44, encode/finish_wall 7.35 |
| measure | 18.16 | 32.0 | main/busy/ms 40.45, stage/main/update 25.51, render/thread/ms 21.61, stream/drive/ms 21.61, encode//sum 21.27, render/busy/ms 20.54, stage/render/cleanup 13.18, stage/main/post_update 12.35, render/graph/ms 11.54, stage/render/graph_submit 9.14, encode/submit 4.77, encode/finish_wall 4.43 |
| measure | 59.05 | 31.4 | render/thread/ms 41.45, render/busy/ms 39.58, encode//sum 36.20, main/busy/ms 29.73, render/graph/ms 22.64, stage/render/graph_submit 18.18, stage/main/post_update 16.35, stage/render/cleanup 13.48, stage/render/render 11.52, encode/finish_wall 10.45, stage/main/update 8.50, encode/submit 7.96 |
| measure | 59.11 | 32.5 | render/thread/ms 36.49, render/busy/ms 35.38, encode//sum 19.54, main/busy/ms 18.45, stage/render/prepare_assets 18.04, stage/render/cleanup 12.01, render/graph/ms 10.45, stage/main/post_update 9.77, stage/render/graph_submit 8.25, stage/main/update 4.75, encode/finish_wall 4.31, encode/submit 4.01 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\b2_off\balmora_walk@2560x1440_off=flatnormal-2 --bench-baseline none --off flatnormal
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 12% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **82%** (min 42%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.0 W · 74 °C (max 79) · VRAM 6906 MiB · throttled 100% of samples
