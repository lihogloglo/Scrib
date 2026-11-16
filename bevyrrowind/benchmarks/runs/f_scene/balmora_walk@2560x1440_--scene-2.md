# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5845 frames over 65.6 s · **median 11.01 ms (91 fps)** · mean 11.23 · p95 13.36 · p99 15.24 (1% low 66 fps) · worst 34.16 ms · 32 spikes over 1.5× median · 22.9 stutters/min (work 21.0/min)
- warm-up: 355 frames over 9.2 s, worst 2512.0 ms
- hitches (> 27.5 ms): **3** measured, 32 in warm-up
- timed GPU passes: 7.33 ms of a 11.23 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.01 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- **FAIL** worst frame 34.16 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.015 | 2.080 | 2.212 | 0.97 | 0.173 |
| main_opaque_pass_3d | 1.568 | 1.619 | 1.973 | 0.97 | 0.058 |
| early prepass | 0.563 | 0.581 | 0.822 | 0.97 | 0.081 |
| bloom | 0.311 | 0.321 | 0.327 | 0.97 | 0.062 |
| atmosphere_luts | 0.304 | 0.313 | 0.372 | 0.97 | 0.003 |
| water_velocity | 0.277 | 0.286 | 0.354 | 0.97 | 0.001 |
| water_sspr | 0.200 | 0.207 | 0.206 | 0.97 | 0.002 |
| ssao | 0.187 | 0.193 | 0.195 | 0.97 | 0.006 |
| atmos_froxel_inject | 0.142 | 0.147 | 0.147 | 0.97 | 0.001 |
| clouds_trace | 0.138 | 0.142 | 0.236 | 0.97 | 0.001 |
| per_view_shadow_pass_cascade1 | 0.135 | 0.577 | 0.649 | 0.23 | 0.042 |
| water_fft | 0.129 | 0.133 | 0.138 | 0.97 | 0.003 |
| auxiliary_views | 0.118 | 0.122 | 0.500 | 0.97 | 0.113 |
| upscaling | 0.115 | 0.119 | 0.127 | 0.97 | 0.004 |
| gpu_instance_materialization | 0.115 | 0.118 | 0.425 | 0.97 | 0.004 |
| uniform_allocation | 0.113 | 0.117 | 0.144 | 0.97 | 0.020 |
| clouds_composite | 0.105 | 0.108 | 0.110 | 0.97 | 0.002 |
| bin_unpacking | 0.095 | 0.098 | 0.116 | 0.97 | 0.016 |
| tonemapping | 0.084 | 0.087 | 0.090 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.082 | 0.084 | 0.161 | 0.97 | 0.008 |
| clustering | 0.065 | 0.067 | 0.069 | 0.97 | 0.018 |
| early_mesh_preprocessing | 0.063 | 0.065 | 0.087 | 0.97 | 0.003 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.423 | 0.421 | 0.11 | 0.001 |
| surface_trail | 0.041 | 0.058 | 0.057 | 0.71 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.97 | 0.001 |
| range_unpacking | 0.025 | 0.026 | 0.029 | 0.97 | 0.003 |
| lightprobe_radiance_map | 0.025 | 0.223 | 0.234 | 0.11 | 0.004 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.97 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.027 | 0.97 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| main_transparent_pass_3d | 0.009 | 0.009 | 0.022 | 0.97 | 0.079 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.020 | 0.016 | 0.11 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.017 | 0.022 | 0.01 | 0.009 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.015 | 0.017 | 0.00 | 0.008 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.474 | 15.152 | 47.65 | 61221.8 |
| render/thread/ms | 10.053 | 12.038 | 29.97 | 58760.1 |
| render/busy/ms | 9.467 | 11.481 | 29.29 | 55333.7 |
| main/busy/ms | 8.837 | 10.928 | 30.54 | 51650.2 |
| stage/render/render | 6.364 | 7.717 | 15.85 | 37196.3 |
| render/graph/ms | 5.604 | 6.941 | 14.73 | 32753.4 |
| stage/main/post_update | 5.287 | 7.081 | 9.45 | 30900.5 |
| stage/render/graph_submit | 4.519 | 5.643 | 11.02 | 26412.6 |
| encode/finish_wall | 2.358 | 3.094 | 6.01 | 13780.1 |
| encode/submit | 2.220 | 2.981 | 6.48 | 12973.4 |
| stage/main/update | 2.192 | 2.681 | 24.84 | 12812.6 |
| stage/render/camera_driver | 1.026 | 1.337 | 3.73 | 5998.2 |
| stage/render/prepare_write_phase_buffers | 0.670 | 0.928 | 1.80 | 3913.9 |
| bigspace/total/ms | 0.614 | 2.074 | 3.62 | 3587.9 |
| stage/render/prepare_assets | 0.544 | 0.696 | 11.89 | 3178.7 |
| render/present/ms | 0.495 | 0.622 | 2.43 | 2895.1 |
| stage/render/prepare_resources | 0.446 | 0.536 | 1.37 | 2608.3 |
| stage/main/first | 0.399 | 0.526 | 4.94 | 2333.8 |
| stage/render/prepare_meshes | 0.385 | 0.495 | 2.88 | 2248.8 |
| stage/main/pre_update | 0.322 | 0.479 | 1.96 | 1880.5 |
| terrain/update/ms | 0.311 | 0.410 | 2.45 | 1816.2 |
| stage/render/prepare_bind_groups | 0.296 | 0.480 | 2.09 | 1732.8 |
| stage/render/prepare_flush | 0.280 | 0.479 | 2.04 | 1638.4 |
| bigspace/low_precision/ms | 0.280 | 1.120 | 3.23 | 1637.5 |
| terrain/select/ms | 0.277 | 0.368 | 0.89 | 1619.7 |
| controller/feed/ms | 0.274 | 0.351 | 0.87 | 1602.6 |
| stage/main/last | 0.247 | 0.300 | 1.80 | 1444.6 |
| stage/render/specialize | 0.245 | 0.361 | 2.34 | 1433.9 |
| stream/drive/ms | 0.244 | 0.326 | 11.33 | 1425.2 |
| stage/main/fixed_main_loop | 0.214 | 0.359 | 1.12 | 1252.8 |
| stage/render/queue | 0.213 | 0.333 | 2.00 | 1243.0 |
| anim/evaluate/ms | 0.206 | 0.339 | 2.95 | 1204.8 |
| stage/render/prepare_batch_phases | 0.200 | 0.285 | 0.51 | 1171.6 |
| bigspace/high_precision/ms | 0.191 | 0.752 | 3.15 | 1113.8 |
| stream/tick/ms | 0.190 | 0.256 | 0.63 | 1112.7 |
| far/select/ms | 0.184 | 0.244 | 0.54 | 1078.0 |
| stage/render/extract_commands | 0.171 | 0.206 | 0.45 | 1002.4 |
| stage/render/create_views | 0.169 | 0.219 | 7.96 | 987.2 |
| anim/evaluate_cpu/ms | 0.145 | 0.187 | 0.45 | 846.4 |
| stage/render/prepare_views | 0.132 | 0.166 | 0.45 | 772.3 |
| controller/move/ms | 0.125 | 0.213 | 0.53 | 732.9 |
| render/acquire/ms | 0.091 | 0.118 | 0.41 | 531.4 |
| pipelines/compile_ms | 0.062 | 0.000 | 115.42 | 364.7 |
| pipelines/front_ms | 0.060 | 0.000 | 111.46 | 350.5 |
| physics/step/ms | 0.047 | 0.083 | 0.56 | 277.2 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.072 | 0.27 | 274.6 |
| stage/render/phase_sort | 0.034 | 0.066 | 0.26 | 198.8 |
| physics/insert/ms | 0.033 | 0.357 | 1.23 | 195.6 |
| encode/resolve_diagnostics/sum | 0.029 | 0.040 | 0.48 | 170.0 |
| weather/map/ms | 0.025 | 0.033 | 0.27 | 145.1 |
| stage/render/cleanup | 0.024 | 0.035 | 0.79 | 139.0 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.26 | 73.6 |
| anim/apply/ms | 0.011 | 0.019 | 0.09 | 66.3 |
| controller/avoid/ms | 0.011 | 0.014 | 0.27 | 62.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 59.9 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.31 | 58.3 |
| script/update/ms | 0.007 | 0.009 | 0.13 | 38.6 |
| script/ui/ms | 0.005 | 0.007 | 0.09 | 29.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.00 | 29.0 |
| script/input/ms | 0.005 | 0.006 | 0.09 | 27.5 |
| stream/upload/ms | 0.004 | 0.000 | 1.27 | 21.8 |
| controller/write/ms | 0.004 | 0.004 | 0.06 | 20.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.05 | 18.1 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.00 | 14.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 14.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.30 | 6.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.3 |
| stage/render/prepare | 0.000 | 0.002 | 0.03 | 2.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.14 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.12 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 564 | 579 | 578 |
| anim/characters/count | count | 566 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1272 | 2262 | 593 |
| crowd/c0/meshes/count | count | 349 | 646 | 156 |
| crowd/c1/count | count | 145 | 154 | 148 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5603 |
| crowd/c1/meshes/count | count | 1167 | 1256 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4618 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4597 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.7 k | 56.7 k | 46.9 k |
| encode//max | ms | 1.94 | 4.11 | 2.02 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.26 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.31 | 0.02 |
| encode/finish_critical | ms | 1.94 | 4.11 | 2.02 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.48 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.51 | 30.54 | 8.86 |
| gpu/timed/ms | ms | 8.69 | 10.75 | 8.86 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 299.7 k | 299.7 k | 299.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.9 k | 13.1 k | 11.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 156 | 387 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 325 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 156 | 387 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 325 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 23 | 22 |
| phases/opaque3d/bins | count | 765 | 971 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2531 | 3590 | 2454 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 765 | 971 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2531 | 3590 | 2454 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 175 | 9 |
| phases/shadow/bins | count | 1571 | 2271 | 897 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 5021 | 6525 | 3791 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 154 | 209 | 21 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2657 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 302 | 303 | 303 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 18 | 498 | 7 |
| preprocess/bind_groups_created | count | 13 | 492 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 125 | 204 | 125 |
| process/cpu_usage | % | 25.96 | 36.06 | 25.96 |
| process/mem_usage | GiB | 5.39 | 5.56 | 5.56 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 6447 | 6571 | 6571 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.30 | 50.12 | 32.75 |
| system/mem_usage | % | 50.56 | 51.31 | 50.91 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2881 | 2996 | 2996 |
| textures/images | count | 2665 | 2723 | 2723 |
| views/all/batch_sets | count | 76 | 230 | 65 |
| views/all/bins | count | 3548 | 4279 | 3179 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 10.8 k | 13.4 k | 9731 |
| views/all/pipelines | count | 198 | 255 | 65 |
| views/all/pipelines_distinct | count | 68 | 72 | 59 |
| views/all/work_items | count | 154.1 k | 209.4 k | 153.8 k |
| views/cascade/batch_sets | count | 13 | 40 | 9 |
| views/cascade/bins | count | 1142 | 1579 | 897 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 4301 | 5541 | 3791 |
| views/cascade/pipelines | count | 25 | 34 | 21 |
| views/cascade/work_items | count | 49.6 k | 103.7 k | 49.3 k |
| views/lamp_face/batch_sets | count | 0 | 143 | 0 |
| views/lamp_face/bins | count | 430 | 753 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 672 | 1093 | 0 |
| views/lamp_face/pipelines | count | 129 | 177 | 0 |
| views/lamp_face/work_items | count | 0 | 711 | 0 |
| views/main/batch_sets | count | 56 | 61 | 56 |
| views/main/bins | count | 2022 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5954 | 7600 | 5940 |
| views/main/pipelines | count | 45 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.6 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 60.30 | 220 | 352.5 k |
| anim/events/count | count | 0.09 | 3 | 554 |
| assets/Image/added | count | 0.10 | 43 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.49 | 352 | 2874 |
| assets/Mesh/removed | count | 0.83 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.71 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.25 | 1492 | 1489 |
| encode//count | count | 39.15 | 276 | 228.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5845 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5845 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5845 |
| physics/inserts | count | 0.38 | 107 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 6 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 6 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5792 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1270 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.5 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2512.0 | main/busy/ms 2486, stage/main/post_update 2386, stage/main/update 94.58, surface/advance_cpu_ms 65.71, stage/main/pre_update 2.41, stage/main/last 1.13, surface/cell_upload_ms 0.69, stage/main/first 0.50, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.29, weather/map/ms 0.13, bigspace/total/ms 0.09 |
| warm-up | 3.24 | 728.7 | main/busy/ms 2634, stage/main/post_update 2415, render/thread/ms 698, render/busy/ms 531, stage/render/prepare_assets 497, stage/main/update 204, stage/render/prepare_views 163, render/acquire/ms 162, surface/advance_cpu_ms 65.71, stage/render/prepare_resources 21.52, stage/render/render 12.89, encode//sum 8.26 |
| warm-up | 3.33 | 89.9 | render/thread/ms 772, render/busy/ms 603, stage/render/prepare_assets 545, main/busy/ms 171, stage/render/prepare_views 163, render/acquire/ms 162, pipelines/compile_ms 140, pipelines/front_ms 125, stage/main/update 115, stage/main/post_update 43.41, stage/render/render 28.06, stage/render/prepare_resources 26.83 |
| warm-up | 3.52 | 166.1 | pipelines/wait_ms 6556, pipelines/compile_ms 1879, pipelines/front_ms 1738, render/thread/ms 163, render/busy/ms 154, pipelines/driver_ms 141, main/busy/ms 65.47, stage/render/prepare_resources 48.77, stage/main/post_update 45.54, stage/render/prepare_flush 32.08, stage/render/prepare_write_phase_buffers 31.93, stage/render/render 18.94 |
| warm-up | 3.63 | 88.8 | pipelines/compile_ms 596, pipelines/front_ms 540, pipelines/wait_ms 472, main/busy/ms 128, render/thread/ms 102, render/busy/ms 99.38, stage/main/update 86.69, stage/render/prepare_assets 73.02, pipelines/driver_ms 55.84, stage/render/render 16.93, stage/main/post_update 16.51, encode//sum 15.85 |
| warm-up | 3.74 | 109.0 | pipelines/compile_ms 1828, pipelines/front_ms 1731, pipelines/wait_ms 482, render/thread/ms 162, render/busy/ms 160, stage/render/prepare_assets 143, main/busy/ms 141, stage/main/update 96.41, pipelines/driver_ms 96.12, stage/main/post_update 18.73, stage/main/fixed_main_loop 12.12, stage/render/render 10.12 |
| warm-up | 3.80 | 34.8 | pipelines/compile_ms 1005, pipelines/front_ms 948, pipelines/wait_ms 107, pipelines/driver_ms 57.09, render/thread/ms 53.93, render/busy/ms 39.23, main/busy/ms 28.00, stage/render/render 26.64, stage/render/prepare_assets 18.37, stage/main/post_update 14.99, render/present/ms 14.28, terrain/update/ms 11.58 |
| warm-up | 3.94 | 82.7 | pipelines/compile_ms 610, pipelines/front_ms 581, main/busy/ms 96.44, render/thread/ms 67.84, render/busy/ms 63.73, stage/main/update 52.47, stage/render/prepare_assets 34.48, stage/main/pre_update 32.28, pipelines/driver_ms 29.18, stage/render/render 24.89, render/graph/ms 20.26, stage/render/graph_submit 16.41 |
| warm-up | 4.00 | 68.8 | pipelines/compile_ms 1552, pipelines/front_ms 1485, render/thread/ms 80.52, render/busy/ms 78.39, pipelines/driver_ms 67.24, main/busy/ms 63.91, stage/render/prepare_assets 54.91, stage/main/pre_update 28.98, stage/main/update 23.75, stage/render/render 18.90, render/graph/ms 16.19, stage/render/graph_submit 12.48 |
| warm-up | 4.08 | 75.3 | pipelines/compile_ms 1286, pipelines/front_ms 1093, pipelines/driver_ms 193, render/thread/ms 100, render/busy/ms 56.80, stage/render/render 51.77, render/present/ms 42.99, stage/render/prepare_assets 42.88, main/busy/ms 24.15, stage/main/update 9.54, stage/main/post_update 8.96, render/graph/ms 8.33 |
| warm-up | 4.12 | 28.1 | pipelines/compile_ms 545, pipelines/front_ms 468, pipelines/driver_ms 77.10, render/thread/ms 29.99, render/busy/ms 28.34, main/busy/ms 23.83, stage/render/render 20.22, render/graph/ms 17.95, encode//sum 17.30, stage/render/graph_submit 15.44, stage/main/post_update 10.76, encode/submit 9.14 |
| warm-up | 4.18 | 48.7 | pipelines/compile_ms 652, pipelines/front_ms 629, main/busy/ms 48.93, render/thread/ms 40.26, render/busy/ms 38.10, stage/main/update 26.46, pipelines/driver_ms 22.78, stage/main/post_update 18.30, stream/drive/ms 18.29, stage/render/render 16.31, render/graph/ms 13.45, encode//sum 12.94 |
| warm-up | 4.24 | 57.3 | pipelines/compile_ms 845, pipelines/front_ms 815, render/thread/ms 67.81, render/busy/ms 65.64, main/busy/ms 49.21, stage/render/prepare_assets 33.75, pipelines/driver_ms 29.38, stage/main/update 24.20, stage/main/post_update 17.51, stage/render/render 17.47, render/graph/ms 14.61, encode//sum 12.18 |
| warm-up | 4.30 | 61.5 | pipelines/compile_ms 450, pipelines/front_ms 435, main/busy/ms 165, stage/main/update 148, render/thread/ms 84.31, render/busy/ms 82.40, stage/render/prepare_assets 52.28, stage/render/render 15.67, pipelines/driver_ms 15.17, render/graph/ms 13.26, encode//sum 9.32, stage/main/post_update 8.54 |
| warm-up | 4.50 | 196.0 | pipelines/compile_ms 2687, pipelines/front_ms 2617, main/busy/ms 151, stage/main/update 137, render/thread/ms 114, render/busy/ms 113, pipelines/driver_ms 70.00, stage/render/prepare_assets 61.23, stage/render/prepare_flush 30.39, stage/render/render 12.00, render/graph/ms 10.03, encode//sum 8.66 |
| warm-up | 4.54 | 37.0 | pipelines/compile_ms 2548, pipelines/front_ms 2477, render/thread/ms 109, render/busy/ms 107, pipelines/driver_ms 71.00, main/busy/ms 45.02, stage/render/prepare_assets 36.68, stage/render/prepare_flush 31.21, stage/main/update 28.36, stage/render/render 27.82, render/graph/ms 24.89, stage/render/graph_submit 19.18 |
| warm-up | 4.60 | 64.8 | pipelines/compile_ms 169, pipelines/front_ms 157, render/thread/ms 92.52, render/busy/ms 90.26, main/busy/ms 64.73, pipelines/wait_ms 30.99, stage/render/render 29.97, stage/main/update 29.82, stage/main/post_update 27.21, render/graph/ms 26.74, stage/render/prepare_assets 26.31, bigspace/total/ms 19.74 |
| warm-up | 4.69 | 85.0 | pipelines/compile_ms 743, pipelines/front_ms 719, pipelines/wait_ms 133, render/thread/ms 132, render/busy/ms 128, main/busy/ms 94.03, stage/main/post_update 46.14, stage/main/update 42.25, stage/render/prepare_assets 36.14, stage/render/prepare_resources 29.62, pipelines/driver_ms 23.79, encode//sum 21.68 |
| warm-up | 4.76 | 72.8 | pipelines/compile_ms 2412, pipelines/front_ms 2338, pipelines/wait_ms 255, render/thread/ms 122, render/busy/ms 118, main/busy/ms 84.37, pipelines/driver_ms 73.76, stage/main/update 44.23, stage/render/prepare_assets 37.44, stage/main/post_update 30.41, encode//sum 28.59, stage/render/prepare_resources 25.74 |
| warm-up | 4.82 | 62.7 | pipelines/compile_ms 1971, pipelines/front_ms 1908, pipelines/wait_ms 164, render/thread/ms 107, render/busy/ms 104, pipelines/driver_ms 62.89, main/busy/ms 62.29, encode//sum 46.95, stage/render/prepare_assets 40.09, stage/render/render 34.95, stage/main/update 33.89, render/graph/ms 31.03 |
| warm-up | 4.88 | 60.2 | pipelines/compile_ms 1071, pipelines/front_ms 1041, pipelines/wait_ms 230, render/thread/ms 103, render/busy/ms 100, main/busy/ms 74.94, stage/main/update 51.19, encode//sum 38.59, stage/render/prepare_assets 36.33, stage/render/render 29.90, pipelines/driver_ms 29.87, render/graph/ms 26.46 |
| warm-up | 4.95 | 64.2 | pipelines/compile_ms 1229, pipelines/front_ms 1197, pipelines/wait_ms 210, render/thread/ms 86.21, render/busy/ms 84.55, main/busy/ms 55.83, stage/render/prepare_assets 47.77, stage/main/update 39.37, pipelines/driver_ms 31.61, encode//sum 16.74, stage/render/render 15.25, render/graph/ms 13.11 |
| warm-up | 5.05 | 84.5 | pipelines/compile_ms 693, pipelines/front_ms 668, main/busy/ms 96.13, stage/main/update 75.61, stream/drive/ms 55.71, render/thread/ms 49.99, render/busy/ms 48.45, pipelines/driver_ms 24.88, encode//sum 16.02, stage/render/prepare_assets 15.64, stage/render/render 14.00, render/graph/ms 11.95 |
| warm-up | 5.09 | 35.8 | pipelines/compile_ms 517, pipelines/front_ms 499, render/thread/ms 59.38, main/busy/ms 55.64, render/busy/ms 55.43, stage/main/update 36.07, stage/render/prepare_assets 18.56, pipelines/driver_ms 18.02, encode//sum 16.27, stage/render/render 15.11, stage/main/post_update 13.33, stream/drive/ms 10.68 |
| warm-up | 5.12 | 28.9 | pipelines/compile_ms 610, pipelines/front_ms 590, render/thread/ms 53.57, render/busy/ms 49.69, main/busy/ms 45.35, stage/main/update 26.35, pipelines/driver_ms 20.07, stage/render/prepare_assets 17.89, encode//sum 17.48, stage/render/render 13.99, stage/main/post_update 12.32, render/graph/ms 9.19 |
| warm-up | 5.33 | 36.0 | pipelines/compile_ms 119, pipelines/front_ms 114, render/thread/ms 53.79, render/busy/ms 52.17, main/busy/ms 45.30, stage/main/update 30.95, stream/drive/ms 21.84, encode//sum 21.48, stage/render/prepare_assets 19.74, stage/render/render 15.92, render/graph/ms 13.78, stage/render/graph_submit 10.44 |
| warm-up | 6.23 | 29.4 | render/thread/ms 43.57, render/busy/ms 42.06, main/busy/ms 34.65, encode//sum 21.57, stage/main/update 18.39, stage/render/render 16.68, render/graph/ms 14.59, stage/render/prepare_assets 11.89, stage/main/post_update 11.85, stage/render/graph_submit 10.58, encode/submit 6.07, encode/finish_wall 4.58 |
| warm-up | 6.31 | 34.4 | render/thread/ms 52.10, render/busy/ms 49.86, main/busy/ms 36.65, encode//sum 28.35, stage/render/render 20.89, stage/main/update 19.51, render/graph/ms 17.76, stage/render/graph_submit 13.47, stage/main/post_update 12.92, stage/render/prepare_assets 12.51, encode/submit 7.86, encode/finish_wall 5.80 |
| warm-up | 6.37 | 30.8 | pipelines/compile_ms 78.09, pipelines/front_ms 74.66, render/thread/ms 39.02, render/busy/ms 37.84, main/busy/ms 29.60, stage/render/prepare_assets 19.64, encode//sum 17.39, stage/main/update 17, stage/render/render 10.34, render/graph/ms 8.71, stage/main/post_update 8.57, stage/render/graph_submit 6.47 |
| warm-up | 6.48 | 33.4 | main/busy/ms 43.73, stage/main/update 29.13, render/thread/ms 28.23, encode//sum 27.45, render/busy/ms 26.96, stage/render/render 15.77, stream/drive/ms 13.89, render/graph/ms 13.76, stage/main/post_update 10.95, stage/render/graph_submit 10.62, encode/finish_wall 5.76, encode/submit 4.90 |
| warm-up | 6.51 | 34.4 | main/busy/ms 47.81, render/thread/ms 45.59, render/busy/ms 44.34, encode//sum 32.16, stage/main/update 30.14, stage/render/render 17.69, render/graph/ms 15.55, stage/render/prepare_assets 13.04, stage/main/post_update 12.39, stage/render/graph_submit 10.99, encode/finish_wall 6.25, encode/submit 4.76 |
| warm-up | 6.54 | 28.6 | render/thread/ms 47.46, main/busy/ms 46.67, render/busy/ms 46.25, stage/main/update 28.89, encode//sum 24.64, stage/render/prepare_assets 19.94, stage/render/render 14.01, render/graph/ms 12.28, stage/main/post_update 10.99, stage/render/graph_submit 8.71, stream/drive/ms 8.36, encode/finish_wall 5.40 |
| measure | 8.14 | 31.0 | encode//sum 52.83, render/thread/ms 39.37, render/busy/ms 38.12, stage/render/render 21.17, render/graph/ms 19.33, main/busy/ms 16.17, stage/render/graph_submit 14.84, stage/main/post_update 9.64, stage/render/create_views 8.14, encode/submit 7.51, encode/finish_wall 7.38, stage/render/camera_driver 4.35 |
| measure | 18.16 | 34.2 | main/busy/ms 41.78, stage/main/update 27.04, render/thread/ms 23.72, render/busy/ms 22.56, encode//sum 21.05, stage/render/render 13.49, render/graph/ms 11.94, stage/main/post_update 11.57, stage/render/graph_submit 9.54, encode/submit 5.22, encode/finish_wall 4.37, stage/render/prepare_assets 3.88 |
| measure | 59.03 | 32.2 | render/thread/ms 38.91, render/busy/ms 37.66, main/busy/ms 23.76, stage/render/prepare_assets 18.56, encode//sum 17.09, stage/main/post_update 11.84, stage/render/render 11.39, render/graph/ms 9.94, stage/render/graph_submit 7.79, stage/main/update 6.67, encode/finish_wall 4.15, encode/submit 3.68 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\f_scene\balmora_walk@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **80%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.8 W · 73 °C (max 78) · VRAM 5250 MiB · throttled 100% of samples
